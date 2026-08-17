use std::{
    path::Path,
    sync::{Arc, Mutex, OnceLock},
};

use fluidvoice_transcription::{TranscriptionConfig, WhisperTranscriber};

pub type SharedTranscriber = Arc<WhisperTranscriber>;

static CACHE: OnceLock<Mutex<Option<(String, SharedTranscriber)>>> = OnceLock::new();

pub fn get(model: &Path, language: &str, use_gpu: bool) -> Result<SharedTranscriber, String> {
    let key = format!("{}\u{1f}{}\u{1f}{use_gpu}", model.display(), language);
    let cache = CACHE.get_or_init(|| Mutex::new(None));
    let mut cached = cache
        .lock()
        .map_err(|_| "Whisper model cache lock was poisoned".to_owned())?;
    if let Some(transcriber) = cached
        .as_ref()
        .filter(|(cached_key, _)| cached_key == &key)
        .map(|(_, transcriber)| Arc::clone(transcriber))
    {
        return Ok(transcriber);
    }

    let config = TranscriptionConfig::default()
        .with_language(Some(language.to_owned()))
        .with_gpu(use_gpu);
    let transcriber =
        Arc::new(WhisperTranscriber::load(model, config).map_err(|error| error.to_string())?);
    cached.replace((key, Arc::clone(&transcriber)));
    Ok(transcriber)
}
