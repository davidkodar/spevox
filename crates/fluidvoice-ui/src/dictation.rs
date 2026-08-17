use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use fluidvoice_audio::MonoAudioBuffer;
use fluidvoice_transcription::{LocalSpeechServer, Transcript};

use crate::{parakeet, whisper_cache};

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
