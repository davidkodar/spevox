use std::{
    path::Path,
    sync::{Arc, Mutex},
    time::Instant,
};

use fluidvoice_audio::MonoAudioBuffer;
use fluidvoice_transcription::{LocalSpeechServer, Transcript};

use crate::{
    ai::{self, AiConfig},
    parakeet, whisper_cache,
};

use super::{ParakeetBackend, native_language_for_model};

pub(super) struct FinalAsrRequest<'a> {
    pub(super) audio: &'a MonoAudioBuffer,
    pub(super) language: &'a str,
    pub(super) speech_engine: i32,
    pub(super) whisper_model: Option<&'a Path>,
    pub(super) use_gpu: bool,
    pub(super) local_speech_url: &'a str,
    pub(super) native_model: Option<parakeet::Model>,
    pub(super) native_backend: ParakeetBackend,
    pub(super) native_supervisor: &'a Arc<Mutex<parakeet::Supervisor>>,
}

pub(super) struct FinalAsrResult {
    pub(super) transcription: Result<Transcript, String>,
    pub(super) native_fallback_error: Option<String>,
}

pub(super) struct EnhancementResult {
    pub(super) result: Option<Result<String, String>>,
    pub(super) duration_ms: u128,
}

pub(super) fn enhance_transcript(
    config: &AiConfig,
    transcript: &str,
    on_update: impl FnMut(&str),
) -> EnhancementResult {
    if !config.enabled {
        return EnhancementResult {
            result: None,
            duration_ms: 0,
        };
    }
    let started = Instant::now();
    let result = ai::enhance_streaming(config, transcript, on_update);
    EnhancementResult {
        result: Some(result),
        duration_ms: started.elapsed().as_millis(),
    }
}

pub(super) fn transcribe_final(
    request: &FinalAsrRequest<'_>,
    on_native_fallback: impl FnOnce(),
) -> FinalAsrResult {
    let whisper_transcription = || {
        request
            .whisper_model
            .ok_or_else(|| "No Whisper model is installed".to_owned())
            .and_then(|model| {
                whisper_cache::get(model, request.use_gpu)?
                    .transcribe_in_language(
                        request.audio,
                        (!request.language.is_empty()).then_some(request.language),
                    )
                    .map_err(|error| error.to_string())
            })
    };

    if request.speech_engine == 5 {
        return FinalAsrResult {
            transcription: LocalSpeechServer::new(request.local_speech_url)
                .and_then(|server| {
                    server.transcribe(
                        request.audio,
                        (!request.language.is_empty()).then_some(request.language),
                    )
                })
                .map_err(|error| error.to_string()),
            native_fallback_error: None,
        };
    }

    let Some(native_model) = request.native_model else {
        return FinalAsrResult {
            transcription: whisper_transcription(),
            native_fallback_error: None,
        };
    };

    let native_language = native_language_for_model(native_model, request.language);
    let primary = request
        .native_supervisor
        .lock()
        .map_err(|_| "Parakeet supervisor lock was poisoned".to_owned())
        .and_then(|mut supervisor| {
            supervisor.ensure_ready(request.native_backend, native_model)?;
            Ok(supervisor.endpoint())
        })
        .and_then(|endpoint| LocalSpeechServer::new(&endpoint).map_err(|error| error.to_string()))
        .and_then(|server| {
            server
                .transcribe(
                    request.audio,
                    (!native_language.is_empty()).then_some(native_language.as_str()),
                )
                .map_err(|error| error.to_string())
        });

    match primary {
        Ok(transcript) => FinalAsrResult {
            transcription: Ok(transcript),
            native_fallback_error: None,
        },
        Err(error) => {
            on_native_fallback();
            FinalAsrResult {
                transcription: whisper_transcription(),
                native_fallback_error: Some(error),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AiConfig, enhance_transcript};

    #[test]
    fn disabled_enhancement_is_an_immediate_noop() {
        let config = AiConfig {
            enabled: false,
            provider: "Ollama".to_owned(),
            model: "unused".to_owned(),
            base_url: "http://127.0.0.1:11434".to_owned(),
            prompt: "unused".to_owned(),
            api_key: String::new(),
            local_only: true,
            timeout_seconds: 30,
        };
        let mut updates = 0;
        let result = enhance_transcript(&config, "unaltered", |_| updates += 1);

        assert!(result.result.is_none());
        assert_eq!(result.duration_ms, 0);
        assert_eq!(updates, 0);
    }
}
