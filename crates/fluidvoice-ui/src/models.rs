#[allow(clippy::too_many_lines)] // Static ISO/Whisper catalog, not control flow.
fn supported_languages() -> &'static [(&'static str, &'static str)] {
    &[
        ("Automatic detection", ""),
        ("Afrikaans", "af"),
        ("Amharic", "am"),
        ("Arabic", "ar"),
        ("Assamese", "as"),
        ("Azerbaijani", "az"),
        ("Bashkir", "ba"),
        ("Belarusian", "be"),
        ("Bulgarian", "bg"),
        ("Bengali", "bn"),
        ("Tibetan", "bo"),
        ("Breton", "br"),
        ("Bosnian", "bs"),
        ("Catalan", "ca"),
        ("Czech", "cs"),
        ("Welsh", "cy"),
        ("Danish", "da"),
        ("German", "de"),
        ("Greek", "el"),
        ("English", "en"),
        ("Spanish", "es"),
        ("Estonian", "et"),
        ("Basque", "eu"),
        ("Persian", "fa"),
        ("Finnish", "fi"),
        ("Faroese", "fo"),
        ("French", "fr"),
        ("Galician", "gl"),
        ("Gujarati", "gu"),
        ("Hausa", "ha"),
        ("Hawaiian", "haw"),
        ("Hebrew", "iw"),
        ("Hindi", "hi"),
        ("Croatian", "hr"),
        ("Haitian Creole", "ht"),
        ("Hungarian", "hu"),
        ("Armenian", "hy"),
        ("Indonesian", "id"),
        ("Icelandic", "is"),
        ("Italian", "it"),
        ("Japanese", "ja"),
        ("Javanese", "jw"),
        ("Georgian", "ka"),
        ("Kazakh", "kk"),
        ("Khmer", "km"),
        ("Kannada", "kn"),
        ("Korean", "ko"),
        ("Latin", "la"),
        ("Luxembourgish", "lb"),
        ("Lingala", "ln"),
        ("Lao", "lo"),
        ("Lithuanian", "lt"),
        ("Latvian", "lv"),
        ("Malagasy", "mg"),
        ("Maori", "mi"),
        ("Macedonian", "mk"),
        ("Malayalam", "ml"),
        ("Mongolian", "mn"),
        ("Marathi", "mr"),
        ("Malay", "ms"),
        ("Maltese", "mt"),
        ("Myanmar", "my"),
        ("Nepali", "ne"),
        ("Dutch", "nl"),
        ("Norwegian Nynorsk", "nn"),
        ("Norwegian", "no"),
        ("Occitan", "oc"),
        ("Punjabi", "pa"),
        ("Polish", "pl"),
        ("Pashto", "ps"),
        ("Portuguese", "pt"),
        ("Romanian", "ro"),
        ("Russian", "ru"),
        ("Sanskrit", "sa"),
        ("Sindhi", "sd"),
        ("Sinhala", "si"),
        ("Slovak", "sk"),
        ("Slovenian", "sl"),
        ("Shona", "sn"),
        ("Somali", "so"),
        ("Albanian", "sq"),
        ("Serbian", "sr"),
        ("Sundanese", "su"),
        ("Swedish", "sv"),
        ("Swahili", "sw"),
        ("Tamil", "ta"),
        ("Telugu", "te"),
        ("Tajik", "tg"),
        ("Thai", "th"),
        ("Turkmen", "tk"),
        ("Tagalog", "tl"),
        ("Turkish", "tr"),
        ("Tatar", "tt"),
        ("Ukrainian", "uk"),
        ("Urdu", "ur"),
        ("Uzbek", "uz"),
        ("Vietnamese", "vi"),
        ("Yiddish", "yi"),
        ("Yoruba", "yo"),
        ("Chinese", "zh"),
    ]
}

#[derive(Clone, Copy)]
struct WhisperModel {
    display_name: &'static str,
    file_name: &'static str,
    expected_bytes: u64,
    sha256: &'static str,
    description: &'static str,
}

fn whisper_model_catalog() -> &'static [WhisperModel] {
    &[
        WhisperModel {
            display_name: "Whisper Tiny",
            file_name: "ggml-tiny.bin",
            expected_bytes: 77_691_713,
            sha256: "be07e048e1e599ad46341c8d2a135645097a538221678b7acdd1b1919c6e1b21",
            description: "75 MB · fastest · basic accuracy · 99 languages",
        },
        WhisperModel {
            display_name: "Whisper Base",
            file_name: "ggml-base.bin",
            expected_bytes: 147_951_465,
            sha256: "60ed5bc3dd14eea856493d334349b405782ddcaf0028d4b5df4088345fba2efe",
            description: "141 MB · fast · balanced for short dictation · 99 languages",
        },
        WhisperModel {
            display_name: "Whisper Small",
            file_name: "ggml-small.bin",
            expected_bytes: 487_601_967,
            sha256: "1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b",
            description: "465 MB · recommended · good accuracy · 99 languages",
        },
        WhisperModel {
            display_name: "Whisper Medium",
            file_name: "ggml-medium.bin",
            expected_bytes: 1_533_763_059,
            sha256: "6c14d5adee5f86394037b4e4e8b59f1673b6cee10e3cf0b11bbdbee79c156208",
            description: "1.4 GB · slower · high accuracy · 6 GB+ RAM",
        },
        WhisperModel {
            display_name: "Whisper Large Turbo",
            file_name: "ggml-large-v3-turbo.bin",
            expected_bytes: 1_624_555_275,
            sha256: "1fc70f774d38eb169993ac391eea357ef47c88757ef72ee5943879b7e8e2bc69",
            description: "1.5 GB · high accuracy · optimized decoding · 8 GB+ RAM",
        },
        WhisperModel {
            display_name: "Whisper Large",
            file_name: "ggml-large-v3.bin",
            expected_bytes: 3_095_033_483,
            sha256: "64d182b440b98d5203c4f9bd541544d84c605196c4f7b845dfa11fb23594d1e2",
            description: "2.9 GB · slowest · highest accuracy · 10 GB+ RAM",
        },
    ]
}

fn managed_model_directory() -> PathBuf {
    if let Some(directory) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(directory).join("fluidvoice/models");
    }
    std::env::var_os("HOME").map_or_else(
        || data_directory().join("models"),
        |home| PathBuf::from(home).join(".local/share/fluidvoice/models"),
    )
}

fn model_search_directories() -> Vec<PathBuf> {
    vec![managed_model_directory()]
}

fn resolve_model_path(model: &WhisperModel) -> PathBuf {
    model_search_directories()
        .into_iter()
        .map(|directory| directory.join(model.file_name))
        .find(|path| model_file_valid(path, model))
        .unwrap_or_else(|| managed_model_directory().join(model.file_name))
}

fn model_ui_lists(paths: &[PathBuf]) -> (QStringList, QStringList) {
    let states = paths
        .iter()
        .zip(whisper_model_catalog())
        .map(|(path, model)| {
            QString::from(if model_file_valid(path, model) {
                "Downloaded"
            } else {
                "Not downloaded"
            })
        })
        .collect();
    let details = whisper_model_catalog()
        .iter()
        .map(|model| QString::from(model.description))
        .collect();
    (states, details)
}

fn model_file_valid(path: &PathBuf, model: &WhisperModel) -> bool {
    fs::metadata(path)
        .is_ok_and(|metadata| metadata.is_file() && metadata.len() == model.expected_bytes)
}

fn download_whisper_model(
    model: WhisperModel,
    destination: &PathBuf,
    cancel: &AtomicBool,
    mut progress: impl FnMut(f32),
) -> Result<(), String> {
    let parent = destination
        .parent()
        .ok_or_else(|| "invalid model destination".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let partial = destination.with_extension("bin.part");
    let url = format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}",
        model.file_name
    );
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_mins(30)))
        .timeout_recv_body(Some(Duration::from_secs(30)))
        .build()
        .new_agent();
    let response = agent.get(&url).call().map_err(|error| error.to_string())?;
    let mut reader = response.into_body().into_reader();
    let mut output = fs::File::create(&partial).map_err(|error| error.to_string())?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; 1024 * 256];
    let mut downloaded = 0_u64;
    loop {
        if cancel.load(Ordering::Relaxed) {
            drop(output);
            fs::remove_file(&partial).ok();
            return Err("cancelled".to_owned());
        }
        let count = reader
            .read(&mut buffer)
            .map_err(|error| error.to_string())?;
        if count == 0 {
            break;
        }
        output
            .write_all(&buffer[..count])
            .map_err(|error| error.to_string())?;
        hasher.update(&buffer[..count]);
        downloaded = downloaded.saturating_add(u64::try_from(count).unwrap_or_default());
        progress(progress_ratio(downloaded, model.expected_bytes));
    }
    output.sync_all().map_err(|error| error.to_string())?;
    drop(output);
    let digest = format!("{:x}", hasher.finalize());
    if downloaded != model.expected_bytes || digest != model.sha256 {
        fs::remove_file(&partial).ok();
        return Err(format!(
            "model verification failed (bytes {downloaded}/{}, sha256 {digest})",
            model.expected_bytes,
        ));
    }
    fs::rename(&partial, destination).map_err(|error| error.to_string())?;
    Ok(())
}
