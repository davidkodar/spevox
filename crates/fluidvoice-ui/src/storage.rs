fn data_directory() -> PathBuf {
    if let Some(directory) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(directory).join("fluidvoice");
    }
    std::env::var_os("HOME").map_or_else(
        || std::env::temp_dir().join(format!("fluidvoice-{}", std::process::id())),
        |home| PathBuf::from(home).join(".local/share/fluidvoice"),
    )
}

fn dictionary_path() -> PathBuf {
    data_directory().join("dictionary.txt")
}

fn history_path() -> PathBuf {
    data_directory().join("history.tsv")
}

fn lifetime_stats_path() -> PathBuf {
    data_directory().join("lifetime-stats.json")
}

#[derive(Default)]
struct LifetimeStats {
    transcript_count: u64,
    dictated_word_count: u64,
}

impl LifetimeStats {
    fn load_or_migrate(history: &[String]) -> Self {
        if let Ok(contents) = fs::read_to_string(lifetime_stats_path()) {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&contents) {
                if let (Some(transcript_count), Some(dictated_word_count)) = (
                    value
                        .get("transcript_count")
                        .and_then(serde_json::Value::as_u64),
                    value
                        .get("dictated_word_count")
                        .and_then(serde_json::Value::as_u64),
                ) {
                    return Self {
                        transcript_count,
                        dictated_word_count,
                    };
                }
            }
        }
        let stats = history
            .iter()
            .filter(|entry| history_field(entry, 7) != Some("file"))
            .fold(Self::default(), |mut stats, entry| {
                stats.transcript_count = stats.transcript_count.saturating_add(1);
                stats.dictated_word_count = stats.dictated_word_count.saturating_add(
                    u64::try_from(
                        history_field(entry, 1)
                            .unwrap_or(entry)
                            .split_whitespace()
                            .count(),
                    )
                    .unwrap_or(u64::MAX),
                );
                stats
            });
        stats.save().ok();
        stats
    }

    fn save(&self) -> Result<(), String> {
        atomic_write_private(
            &lifetime_stats_path(),
            serde_json::to_string_pretty(&serde_json::json!({
                "transcript_count": self.transcript_count,
                "dictated_word_count": self.dictated_word_count,
            }))
            .map_err(|error| error.to_string())?
            .as_bytes(),
        )
    }

    fn transcript_count_i32(&self) -> i32 {
        i32::try_from(self.transcript_count).unwrap_or(i32::MAX)
    }

    fn dictated_word_count_i32(&self) -> i32 {
        i32::try_from(self.dictated_word_count).unwrap_or(i32::MAX)
    }
}

fn audio_history_directory() -> PathBuf {
    data_directory().join("audio-history")
}

fn audio_history_summary() -> String {
    let Ok(entries) = fs::read_dir(audio_history_directory()) else {
        return "No retained recordings · retention is off by default".to_owned();
    };
    let (count, bytes) = entries
        .flatten()
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold((0_u64, 0_u64), |(count, bytes), metadata| {
            (count + 1, bytes.saturating_add(metadata.len()))
        });
    format!(
        "{count} retained recording(s) · {:.1} MB used",
        bytes as f64 / 1_048_576.0
    )
}

fn save_audio_history(
    audio: &fluidvoice_audio::MonoAudioBuffer,
    budget_bytes: u64,
) -> Result<PathBuf, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let directory = audio_history_directory();
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_millis());
    let path = directory.join(format!("dictation-{timestamp}.wav"));
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: audio.sample_rate(),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&path, spec).map_err(|error| error.to_string())?;
    for sample in audio.samples() {
        let value = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16;
        writer
            .write_sample(value)
            .map_err(|error| error.to_string())?;
    }
    writer.finalize().map_err(|error| error.to_string())?;
    prune_audio_history(budget_bytes)?;
    Ok(path)
}

fn prune_audio_history(budget_bytes: u64) -> Result<(), String> {
    let directory = audio_history_directory();
    let Ok(entries) = fs::read_dir(&directory) else {
        return Ok(());
    };
    let mut files = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            metadata
                .is_file()
                .then_some((entry.path(), metadata.len(), metadata.modified().ok()))
        })
        .collect::<Vec<_>>();
    files.sort_by_key(|(_, _, modified)| *modified);
    let mut total = files.iter().map(|(_, size, _)| *size).sum::<u64>();
    for (path, size, _) in files {
        if total <= budget_bytes {
            break;
        }
        fs::remove_file(path).map_err(|error| error.to_string())?;
        total = total.saturating_sub(size);
    }
    clear_missing_audio_history_references()?;
    Ok(())
}

fn clear_missing_audio_history_references() -> Result<(), String> {
    let _history_guard = HISTORY_IO_LOCK
        .lock()
        .map_err(|_| "history lock was poisoned".to_owned())?;
    let path = history_path();
    let mut history = load_lines(&path);
    let mut changed = false;
    for entry in &mut history {
        let mut fields = entry.split('\t').map(str::to_owned).collect::<Vec<_>>();
        if fields.get(8).is_some_and(|audio| !audio.is_empty())
            && !fields
                .get(8)
                .is_some_and(|audio| PathBuf::from(audio).is_file())
        {
            fields[8].clear();
            *entry = fields.join("\t");
            changed = true;
        }
    }
    if changed {
        save_lines(&path, &history).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn write_audio_history_zip(output_path: &PathBuf) -> Result<usize, String> {
    let parent = output_path
        .parent()
        .ok_or_else(|| "export path has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let file = fs::File::create(output_path).map_err(|error| error.to_string())?;
    let mut archive = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o600);
    archive
        .start_file("history.tsv", options)
        .map_err(|error| error.to_string())?;
    archive
        .write_all(&fs::read(history_path()).unwrap_or_default())
        .map_err(|error| error.to_string())?;
    let directory = audio_history_directory();
    let canonical_directory = directory.canonicalize().ok();
    let mut count = 0;
    if let Ok(entries) = fs::read_dir(&directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path == *output_path {
                continue;
            }
            let safe = path
                .canonicalize()
                .ok()
                .zip(canonical_directory.clone())
                .is_some_and(|(path, directory)| path.starts_with(directory));
            if !safe || !path.is_file() {
                continue;
            }
            let Some(name) = path.file_name().and_then(std::ffi::OsStr::to_str) else {
                continue;
            };
            archive
                .start_file(format!("audio/{name}"), options)
                .map_err(|error| error.to_string())?;
            archive
                .write_all(&fs::read(&path).map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())?;
            count += 1;
        }
    }
    archive.finish().map_err(|error| error.to_string())?;
    Ok(count)
}

fn ai_profiles_path() -> PathBuf {
    data_directory().join("ai-profiles.json")
}

fn command_history_path() -> PathBuf {
    data_directory().join("command-history.tsv")
}

fn append_command_history(
    mut controller: Pin<&mut ffi::FluidVoiceController>,
    role: &str,
    text: &str,
) {
    let mut history = load_lines(&command_history_path());
    history.push(format!("{}\t{}", history_value(role), history_value(text)));
    if history.len() > 200 {
        history.drain(..history.len() - 200);
    }
    save_lines(&command_history_path(), &history).ok();
    controller
        .as_mut()
        .set_command_history(history.iter().rev().map(QString::from).collect());
}

fn load_lines(path: &PathBuf) -> Vec<String> {
    fs::read_to_string(path)
        .map(|contents| {
            contents
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn save_lines(path: &PathBuf, lines: &[String]) -> Result<(), String> {
    let mut contents = lines.join("\n");
    if !contents.is_empty() {
        contents.push('\n');
    }
    atomic_write_private(path, contents.as_bytes())
}

fn atomic_write_private(path: &std::path::Path, contents: &[u8]) -> Result<(), String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let parent = path
        .parent()
        .ok_or_else(|| "data path has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::set_permissions(parent, fs::Permissions::from_mode(0o700))
        .map_err(|error| error.to_string())?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let temporary = parent.join(format!(
        ".{}.{}-{nonce}.tmp",
        path.file_name()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("fluidvoice"),
        std::process::id()
    ));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&temporary)
        .map_err(|error| error.to_string())?;
    if let Err(error) = file.write_all(contents).and_then(|()| file.sync_all()) {
        fs::remove_file(&temporary).ok();
        return Err(error.to_string());
    }
    drop(file);
    fs::rename(&temporary, path).map_err(|error| {
        fs::remove_file(&temporary).ok();
        error.to_string()
    })
}

