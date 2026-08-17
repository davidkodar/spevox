#[derive(Clone, Debug, Eq, PartialEq)]
struct MeetingSegment {
    start_milliseconds: u64,
    end_milliseconds: u64,
    speaker: Option<String>,
    text: String,
}

struct MeetingTranscript {
    text: String,
    segments: Vec<MeetingSegment>,
    diarization_warning: Option<String>,
}

// The worker boundary takes an immutable snapshot of every relevant setting;
// a parameter object would only move these one-shot values behind indirection.
#[allow(clippy::too_many_arguments)]
fn transcribe_long_audio_file(
    path: &std::path::Path,
    model: &std::path::Path,
    language: String,
    use_gpu: bool,
    diarization_enabled: bool,
    diarization_backend: ParakeetBackend,
    cancel: &AtomicBool,
    mut progress: impl FnMut(f32, usize, usize),
) -> Result<MeetingTranscript, String> {
    const CHUNK_SAMPLES: usize = 16_000 * 30;
    let audio = decode_audio_file(path)?;
    let config = TranscriptionConfig::default()
        .with_language(Some(language))
        .with_gpu(use_gpu);
    let transcriber = WhisperTranscriber::load(model, config).map_err(|error| error.to_string())?;
    let total = audio.samples().len().div_ceil(CHUNK_SAMPLES);
    let mut segments = Vec::new();
    for (index, samples) in audio.samples().chunks(CHUNK_SAMPLES).enumerate() {
        if cancel.load(Ordering::Relaxed) {
            return Err("Meeting transcription cancelled".to_owned());
        }
        let chunk = AudioBuffer::new(samples.to_vec(), 16_000, 1, false)
            .map_err(|error| error.to_string())?
            .to_asr_mono();
        let transcript = transcriber
            .transcribe(&chunk)
            .map_err(|error| error.to_string())?;
        let chunk_offset_ms = u64::try_from(index)
            .unwrap_or(u64::MAX)
            .saturating_mul(30_000);
        for segment in transcript.segments {
            let start = u64::try_from(segment.start_centiseconds.max(0)).unwrap_or_default() * 10;
            let end = u64::try_from(segment.end_centiseconds.max(0)).unwrap_or_default() * 10;
            segments.push(MeetingSegment {
                start_milliseconds: chunk_offset_ms.saturating_add(start),
                end_milliseconds: chunk_offset_ms.saturating_add(end),
                speaker: None,
                text: segment.text,
            });
        }
        let completed = index + 1;
        progress(
            progress_ratio(
                u64::try_from(completed).unwrap_or(u64::MAX),
                u64::try_from(total).unwrap_or(u64::MAX),
            ),
            completed,
            total,
        );
    }
    let mut diarization_warning = None;
    if diarization_enabled {
        if !parakeet::model_installed(parakeet::SORTFORMER_V2) {
            diarization_warning = Some(
                "Sortformer is not installed; run its one-click setup in File Transcription."
                    .to_owned(),
            );
        } else if !parakeet::runtime_installed(diarization_backend) {
            diarization_warning = Some(
                "the selected native compute runtime is unavailable; run Check setup.".to_owned(),
            );
        } else {
            match write_temporary_diarization_wav(&audio).and_then(|wav| {
                let result = parakeet::diarize_file(diarization_backend, &wav);
                fs::remove_file(wav).ok();
                result
            }) {
                Ok(diarization) => assign_speakers(&mut segments, &diarization),
                Err(error) => {
                    diarization_warning = Some(format!(
                        "experimental diarization failed, so the Whisper transcript was preserved: {error}"
                    ));
                }
            }
        }
    }
    let text = segments
        .iter()
        .map(|segment| segment.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    if text.trim().is_empty() {
        return Err("No speech was recognized in this file.".to_owned());
    }
    Ok(MeetingTranscript {
        text,
        segments,
        diarization_warning,
    })
}

fn write_temporary_diarization_wav(
    audio: &fluidvoice_audio::MonoAudioBuffer,
) -> Result<PathBuf, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let directory = std::env::var_os("XDG_RUNTIME_DIR")
        .map_or_else(std::env::temp_dir, PathBuf::from)
        .join("fluidvoice");
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))
        .map_err(|error| error.to_string())?;
    let path = directory.join(format!(
        "fluidvoice-diarization-{}-{nonce}.wav",
        std::process::id()
    ));
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&path)
        .map_err(|error| error.to_string())?;
    let mut writer = hound::WavWriter::new(file, spec).map_err(|error| error.to_string())?;
    for sample in audio.samples() {
        #[allow(clippy::cast_possible_truncation)]
        let value = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16;
        writer
            .write_sample(value)
            .map_err(|error| error.to_string())?;
    }
    writer.finalize().map_err(|error| error.to_string())?;
    Ok(path)
}

fn assign_speakers(
    transcript: &mut [MeetingSegment],
    diarization: &[parakeet::DiarizationSegment],
) {
    for segment in transcript {
        let start = display_ratio(segment.start_milliseconds, 1_000);
        let end = display_ratio(segment.end_milliseconds, 1_000);
        let speaker = diarization
            .iter()
            .filter_map(|candidate| {
                let overlap = end.min(candidate.end_seconds) - start.max(candidate.start_seconds);
                (overlap > 0.0).then_some((candidate.speaker, overlap))
            })
            .max_by(|left, right| left.1.total_cmp(&right.1))
            .map(|(speaker, _)| format!("Speaker {speaker}"));
        segment.speaker = speaker;
    }
}

fn meeting_speaker_names(segments: &[MeetingSegment]) -> Vec<String> {
    let mut speakers = Vec::new();
    for speaker in segments
        .iter()
        .filter_map(|segment| segment.speaker.as_ref())
    {
        if !speakers.contains(speaker) {
            speakers.push(speaker.clone());
        }
    }
    speakers
}

fn rename_latest_file_history_speaker(current: &str, replacement: &str) -> Result<(), String> {
    let _history_guard = HISTORY_IO_LOCK
        .lock()
        .map_err(|_| "history lock was poisoned".to_owned())?;
    let path = history_path();
    let mut history = load_lines(&path);
    rename_latest_file_history_speaker_entries(&mut history, current, replacement)?;
    save_lines(&path, &history)
}

fn rename_latest_file_history_speaker_entries(
    history: &mut [String],
    current: &str,
    replacement: &str,
) -> Result<(), String> {
    let entry = history
        .iter_mut()
        .rev()
        .find(|entry| history_field(entry, 7) == Some("file"))
        .ok_or_else(|| "no file-transcription History entry exists".to_owned())?;
    let mut fields = entry.split('\t').map(str::to_owned).collect::<Vec<_>>();
    let text = fields
        .get_mut(1)
        .ok_or_else(|| "the latest History entry is incomplete".to_owned())?;
    let needle = format!("{current}: ");
    if !text.contains(&needle) {
        return Err("the latest History entry does not contain that speaker".to_owned());
    }
    *text = text.replace(&needle, &format!("{replacement}: "));
    *entry = fields.join("\t");
    Ok(())
}

fn meeting_segment_qstring(segment: &MeetingSegment) -> QString {
    QString::from(&format!(
        "{}\t{}\t{}\t{}",
        segment.start_milliseconds,
        segment.end_milliseconds,
        segment.speaker.as_deref().unwrap_or(""),
        history_value(&segment.text)
    ))
}

fn timestamp_srt(milliseconds: u64, decimal: char) -> String {
    let hours = milliseconds / 3_600_000;
    let minutes = (milliseconds / 60_000) % 60;
    let seconds = (milliseconds / 1_000) % 60;
    let millis = milliseconds % 1_000;
    format!("{hours:02}:{minutes:02}:{seconds:02}{decimal}{millis:03}")
}

fn write_meeting_export(
    path: &PathBuf,
    format: &str,
    segments: &[MeetingSegment],
) -> Result<(), String> {
    if segments.is_empty() {
        return Err("No timestamped meeting transcript is available".to_owned());
    }
    let format = format.to_ascii_lowercase();
    let contents = match format.as_str() {
        "json" => serde_json::to_string_pretty(
            &segments
                .iter()
                .map(|segment| {
                    serde_json::json!({
                        "start_ms": segment.start_milliseconds,
                        "end_ms": segment.end_milliseconds,
                        "speaker": segment.speaker,
                        "text": segment.text,
                    })
                })
                .collect::<Vec<_>>(),
        )
        .map_err(|error| error.to_string())?,
        "srt" => segments
            .iter()
            .enumerate()
            .map(|(index, segment)| {
                format!(
                    "{}\n{} --> {}\n{}{}\n",
                    index + 1,
                    timestamp_srt(segment.start_milliseconds, ','),
                    timestamp_srt(segment.end_milliseconds, ','),
                    segment
                        .speaker
                        .as_deref()
                        .map_or(String::new(), |speaker| format!("{speaker}: ")),
                    segment.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        "vtt" => format!(
            "WEBVTT\n\n{}",
            segments
                .iter()
                .map(|segment| format!(
                    "{} --> {}\n{}{}\n",
                    timestamp_srt(segment.start_milliseconds, '.'),
                    timestamp_srt(segment.end_milliseconds, '.'),
                    segment
                        .speaker
                        .as_deref()
                        .map_or(String::new(), |speaker| format!("{speaker}: ")),
                    segment.text
                ))
                .collect::<Vec<_>>()
                .join("\n")
        ),
        "md" | "markdown" => segments
            .iter()
            .map(|segment| {
                format!(
                    "- **{}**{} {}",
                    timestamp_srt(segment.start_milliseconds, '.'),
                    segment
                        .speaker
                        .as_deref()
                        .map_or(String::new(), |speaker| format!(" · {speaker}:")),
                    segment.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        "txt" => segments
            .iter()
            .map(|segment| {
                format!(
                    "[{}] {}{}",
                    timestamp_srt(segment.start_milliseconds, '.'),
                    segment
                        .speaker
                        .as_deref()
                        .map_or(String::new(), |speaker| format!("{speaker}: ")),
                    segment.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        _ => return Err(format!("Unsupported meeting export format: {format}")),
    };
    fs::write(path, contents).map_err(|error| error.to_string())
}

fn decode_audio_file(path: &std::path::Path) -> Result<fluidvoice_audio::MonoAudioBuffer, String> {
    const MAX_DECODED_BYTES: u64 = 16_000 * 4 * 60 * 60 * 2;
    let mut command = Command::new("ffmpeg");
    command
        .args(["-nostdin", "-v", "error", "-i"])
        .arg(path)
        .args(["-vn", "-f", "f32le", "-ac", "1", "-ar", "16000", "pipe:1"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    match command.spawn() {
        Ok(mut child) => {
            let mut bytes = Vec::new();
            child
                .stdout
                .take()
                .ok_or_else(|| "FFmpeg stdout was unavailable".to_owned())?
                .take(MAX_DECODED_BYTES + 1)
                .read_to_end(&mut bytes)
                .map_err(|error| error.to_string())?;
            let output = child
                .wait_with_output()
                .map_err(|error| error.to_string())?;
            if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_DECODED_BYTES {
                return Err("Decoded audio exceeds the two-hour safety limit".to_owned());
            }
            if !output.status.success() {
                return Err(format!(
                    "FFmpeg could not decode this audio file: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                ));
            }
            if bytes.len() % 4 != 0 {
                return Err("FFmpeg returned incomplete audio samples".to_owned());
            }
            let samples = bytes
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect::<Vec<_>>();
            return AudioBuffer::new(samples, 16_000, 1, false)
                .map(|audio| audio.to_asr_mono())
                .map_err(|error| error.to_string());
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(format!("Could not start FFmpeg: {error}")),
    }

    let mut reader = hound::WavReader::open(path)
        .map_err(|error| format!("FFmpeg is not installed and WAV fallback failed: {error}"))?;
    let specification = reader.spec();
    if specification.sample_format != hound::SampleFormat::Int
        || specification.bits_per_sample != 16
    {
        return Err("FFmpeg is required for audio other than 16-bit PCM WAV.".to_owned());
    }
    let samples = reader
        .samples::<i16>()
        .map(|sample| sample.map(|value| f32::from(value) / f32::from(i16::MAX)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Could not decode WAV file: {error}"))?;
    let native = AudioBuffer::new(
        samples,
        specification.sample_rate,
        u32::from(specification.channels),
        false,
    )
    .map_err(|error| error.to_string())?;
    Ok(native.to_asr_mono())
}
