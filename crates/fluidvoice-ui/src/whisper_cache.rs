use std::{
    path::Path,
    sync::{Arc, Mutex, OnceLock},
};

use fluidvoice_transcription::{TranscriptionConfig, WhisperTranscriber};

pub(crate) type SharedTranscriber = Arc<WhisperTranscriber>;

static CACHE: OnceLock<Mutex<Option<(String, SharedTranscriber)>>> = OnceLock::new();

pub(crate) fn get(model: &Path, use_gpu: bool) -> Result<SharedTranscriber, String> {
    let metadata = model.metadata().map_err(|error| error.to_string())?;
    let modified = metadata
        .modified()
        .ok()
        .and_then(|value| value.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |value| value.as_nanos());
    let key = format!(
        "{}\u{1f}{use_gpu}\u{1f}{}\u{1f}{modified}",
        model.display(),
        metadata.len()
    );
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

    let config = TranscriptionConfig::default().with_gpu(use_gpu);
    let transcriber =
        Arc::new(WhisperTranscriber::load(model, config).map_err(|error| error.to_string())?);
    cached.replace((key, Arc::clone(&transcriber)));
    Ok(transcriber)
}
