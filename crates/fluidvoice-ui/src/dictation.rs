use std::{
    path::Path,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender, SyncSender},
    },
    thread::JoinHandle,
    time::{Duration, Instant},
};

use fluidvoice_audio::{AudioBuffer, CaptureStopToken, MonoAudioBuffer, PipeWireCapture};
use fluidvoice_delivery::ClipboardDelivery;
use fluidvoice_transcription::{LocalSpeechServer, Transcript};
use tokio::sync::mpsc as tokio_mpsc;

use crate::{
    ai::{self, AiConfig},
    parakeet, whisper_cache,
};

use super::{
    DesktopCommand, ParakeetBackend, asr_gain, native_language_for_model, suspicious_single_word,
};

pub(super) struct PreviewConfig {
    pub(super) speech_engine: i32,
    pub(super) whisper_model: Option<PathBuf>,
    pub(super) language: String,
    pub(super) use_gpu: bool,
    pub(super) gain: f32,
    pub(super) native_model: Option<parakeet::Model>,
    pub(super) native_backend: ParakeetBackend,
    pub(super) native_supervisor: Arc<Mutex<parakeet::Supervisor>>,
}

pub(super) struct PreviewSession {
    preview_sender: SyncSender<AudioBuffer>,
    stream_sender: Option<Sender<AudioBuffer>>,
    stop: Arc<AtomicBool>,
    workers: Vec<JoinHandle<()>>,
}

impl PreviewSession {
    pub(super) fn start(
        config: PreviewConfig,
        publish: impl Fn(String) + Send + Sync + 'static,
    ) -> Self {
        let publish = Arc::new(publish);
        let (preview_sender, preview_receiver) = mpsc::sync_channel::<AudioBuffer>(1);
        let stop = Arc::new(AtomicBool::new(false));
        let native_realtime = config.native_backend == ParakeetBackend::Cpu
            && config.native_model.is_some_and(|model| model.realtime);
        let mut workers = Vec::with_capacity(2);

        let stream_sender =
            if let Some(native_model) = config.native_model.filter(|_| native_realtime) {
                let (stream_sender, stream_receiver) = mpsc::channel::<AudioBuffer>();
                let supervisor = Arc::clone(&config.native_supervisor);
                let language = native_language_for_model(native_model, &config.language);
                let gain = config.gain;
                let backend = config.native_backend;
                let native_publish = Arc::clone(&publish);
                workers.push(std::thread::spawn(move || {
                    let endpoint = supervisor
                        .lock()
                        .map_err(|_| "Native speech supervisor lock was poisoned".to_owned())
                        .and_then(|mut supervisor| {
                            supervisor.ensure_ready(backend, native_model)?;
                            Ok(supervisor.endpoint())
                        });
                    let Ok(endpoint) = endpoint else { return };
                    parakeet::stream_transcript(
                        &endpoint,
                        &stream_receiver,
                        language,
                        gain,
                        move |text| native_publish(text),
                    )
                    .ok();
                }));
                Some(stream_sender)
            } else {
                None
            };
        let worker_stop = Arc::clone(&stop);
        workers.push(std::thread::spawn(move || {
            if config.speech_engine == 5 || native_realtime {
                return;
            }
            let Some(model) = config.whisper_model else {
                return;
            };
            let automatic_language = config.language.is_empty();
            let Ok(transcriber) = whisper_cache::get(&model, config.use_gpu) else {
                return;
            };
            while let Ok(audio) = preview_receiver.recv() {
                if worker_stop.load(Ordering::Acquire) {
                    return;
                }
                let preview_duration = audio.duration();
                if automatic_language && preview_duration < Duration::from_millis(2_500) {
                    continue;
                }
                let mono = audio.to_asr_mono();
                let preview_audio = mono.amplified(asr_gain(mono.peak(), config.gain));
                let Ok(transcript) = transcriber.transcribe_cancellable(
                    &preview_audio,
                    (!config.language.is_empty()).then_some(config.language.as_str()),
                    Arc::clone(&worker_stop),
                ) else {
                    continue;
                };
                if worker_stop.load(Ordering::Acquire)
                    || transcript.text.is_empty()
                    || suspicious_single_word(&transcript.text, preview_duration)
                {
                    continue;
                }
                publish(transcript.text);
            }
        }));
        Self {
            preview_sender,
            stream_sender,
            stop,
            workers,
        }
    }

    pub(super) fn preview_sender(&self) -> SyncSender<AudioBuffer> {
        self.preview_sender.clone()
    }

    pub(super) fn stream_sender(&self) -> Option<Sender<AudioBuffer>> {
        self.stream_sender.clone()
    }

    pub(super) fn stop_and_join(self) {
        self.stop.store(true, Ordering::Release);
        drop(self.preview_sender);
        drop(self.stream_sender);
        for worker in self.workers {
            worker.join().ok();
        }
    }
}

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

pub(super) fn capture_audio(
    target: Option<&str>,
    stop_token: &CaptureStopToken,
    mut on_level: impl FnMut(f32) + 'static,
    on_preview: impl FnMut(AudioBuffer) + 'static,
    on_stream_chunk: impl FnMut(AudioBuffer) + 'static,
) -> Result<AudioBuffer, String> {
    let mut last_level_report: Option<Instant> = None;
    PipeWireCapture::capture_with_streaming_preview(
        Duration::from_mins(2),
        target,
        stop_token,
        move |level| {
            if last_level_report
                .is_some_and(|reported| reported.elapsed() < Duration::from_millis(50))
            {
                return;
            }
            last_level_report = Some(Instant::now());
            on_level(level);
        },
        on_preview,
        on_stream_chunk,
    )
    .map_err(|error| error.to_string())
}

pub(super) fn deliver_transcript(
    clipboard: &mut Option<ClipboardDelivery>,
    desktop_sender: Option<&tokio_mpsc::UnboundedSender<DesktopCommand>>,
    text: &str,
) -> bool {
    if clipboard.is_none() {
        *clipboard = ClipboardDelivery::connect().ok();
    }
    let delivered = clipboard
        .as_mut()
        .is_some_and(|delivery| delivery.copy_transcript(text).is_ok());
    if delivered && let Some(sender) = desktop_sender {
        sender.send(DesktopCommand::Paste).ok();
    }
    delivered
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
