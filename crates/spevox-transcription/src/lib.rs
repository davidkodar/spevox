//! Offline speech-to-text through `whisper.cpp`.

use std::{
    error::Error,
    fmt,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use spevox_audio::{MonoAudioBuffer, encode_pcm16_wav};
use url::{Host, Url};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranscriptionConfig {
    language: Option<String>,
    thread_count: i32,
    use_gpu: bool,
}

impl Default for TranscriptionConfig {
    fn default() -> Self {
        Self {
            language: None,
            thread_count: default_thread_count(),
            use_gpu: true,
        }
    }
}

impl TranscriptionConfig {
    /// Configures a fixed ISO 639-1 language, or automatic detection with `None`.
    #[must_use]
    pub fn with_language(mut self, language: Option<String>) -> Self {
        self.language = language.filter(|value| !value.trim().is_empty());
        self
    }

    #[must_use]
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    #[must_use]
    pub const fn thread_count(&self) -> i32 {
        self.thread_count
    }

    /// Selects the compiled Vulkan backend. When disabled, inference is forced
    /// onto the CPU; when enabled, whisper.cpp falls back to CPU if necessary.
    #[must_use]
    pub const fn with_gpu(mut self, use_gpu: bool) -> Self {
        self.use_gpu = use_gpu;
        self
    }

    #[must_use]
    pub const fn use_gpu(&self) -> bool {
        self.use_gpu
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TranscriptSegment {
    pub start_centiseconds: i64,
    pub end_centiseconds: i64,
    pub text: String,
    pub no_speech_probability: f32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Transcript {
    pub text: String,
    pub segments: Vec<TranscriptSegment>,
    pub detected_language: Option<String>,
}

pub struct WhisperTranscriber {
    context: WhisperContext,
    config: TranscriptionConfig,
}

/// A local OpenAI-compatible speech endpoint, such as a user-managed
/// `whisper.cpp` or `sherpa-onnx` gateway. Network hosts are deliberately
/// rejected so enabling this backend cannot upload microphone audio.
pub struct LocalSpeechServer {
    endpoint: String,
}

impl LocalSpeechServer {
    /// # Errors
    /// Returns an error unless the URL is plain HTTP on loopback.
    pub fn new(base_url: &str) -> Result<Self, TranscriptionError> {
        let mut base = Url::parse(base_url.trim())
            .map_err(|_| TranscriptionError::new("external speech server URL is invalid"))?;
        let host_is_loopback = match base.host() {
            Some(Host::Domain(host)) => host.eq_ignore_ascii_case("localhost"),
            Some(Host::Ipv4(address)) => address.is_loopback(),
            Some(Host::Ipv6(address)) => address.is_loopback(),
            None => false,
        };
        if base.scheme() != "http"
            || !base.username().is_empty()
            || base.password().is_some()
            || !host_is_loopback
            || base.query().is_some()
            || base.fragment().is_some()
        {
            return Err(TranscriptionError::new(
                "external speech server must use HTTP loopback (localhost, 127.0.0.1, or ::1)",
            ));
        }
        if base.path().trim_end_matches('/') != "/v1/audio/transcriptions" {
            base.set_path("/v1/audio/transcriptions");
        }
        let endpoint = base.to_string();
        Ok(Self { endpoint })
    }

    /// Sends bounded mono audio to the explicitly configured local service.
    ///
    /// # Errors
    /// Returns an error for empty/oversized audio, transport failures, or an
    /// invalid OpenAI-compatible JSON response.
    pub fn transcribe(
        &self,
        audio: &MonoAudioBuffer,
        language: Option<&str>,
    ) -> Result<Transcript, TranscriptionError> {
        if audio.samples().is_empty() {
            return Err(TranscriptionError::new("cannot transcribe empty audio"));
        }
        if audio.samples().len() > 16_000 * 60 * 2 {
            return Err(TranscriptionError::new(
                "local speech-server dictation is limited to two minutes",
            ));
        }
        let boundary = "spevox-local-asr-boundary";
        let wav =
            encode_pcm16_wav(audio).map_err(|error| TranscriptionError::new(error.to_string()))?;
        let mut body = Vec::with_capacity(wav.len() + 512);
        append_form_field(&mut body, boundary, "model", "default");
        append_form_field(&mut body, boundary, "response_format", "json");
        if let Some(language) = language.filter(|value| !value.trim().is_empty()) {
            if !(2..=10).contains(&language.len())
                || !language
                    .bytes()
                    .all(|byte| byte.is_ascii_alphabetic() || byte == b'-')
            {
                return Err(TranscriptionError::new(
                    "speech-server language must be a 2–10 character language code",
                ));
            }
            append_form_field(&mut body, boundary, "language", language);
        }
        body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"dictation.wav\"\r\nContent-Type: audio/wav\r\n\r\n").as_bytes());
        body.extend_from_slice(&wav);
        body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_mins(2)))
            .timeout_recv_body(Some(Duration::from_secs(30)))
            .proxy(None)
            .max_redirects(0)
            .build()
            .new_agent();
        let mut response = agent
            .post(&self.endpoint)
            .header(
                "content-type",
                &format!("multipart/form-data; boundary={boundary}"),
            )
            .send(body)
            .map_err(|error| {
                TranscriptionError::new(format!("local speech server failed: {error}"))
            })?;
        let response = response
            .body_mut()
            .read_to_string()
            .map_err(|error| TranscriptionError::new(error.to_string()))?;
        if response.len() > 1_048_576 {
            return Err(TranscriptionError::new(
                "local speech response exceeded 1 MiB",
            ));
        }
        let value: serde_json::Value = serde_json::from_str(&response).map_err(|error| {
            TranscriptionError::new(format!("invalid speech response: {error}"))
        })?;
        let text = value
            .get("text")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_owned();
        if text.is_empty() {
            return Err(TranscriptionError::new(
                "local speech server returned no text",
            ));
        }
        Ok(Transcript {
            text,
            segments: Vec::new(),
            detected_language: value
                .get("language")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
        })
    }
}

fn append_form_field(body: &mut Vec<u8>, boundary: &str, name: &str, value: &str) {
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n"
        )
        .as_bytes(),
    );
}

impl WhisperTranscriber {
    /// Loads a local `whisper.cpp` GGML model into a CPU inference context.
    ///
    /// # Errors
    /// Returns an error if the model path is missing, is not a file, or cannot
    /// be loaded by `whisper.cpp`.
    pub fn load(
        model_path: &Path,
        config: TranscriptionConfig,
    ) -> Result<Self, TranscriptionError> {
        if !model_path.is_file() {
            return Err(TranscriptionError::new(format!(
                "Whisper model does not exist or is not a file: {}",
                model_path.display()
            )));
        }
        let mut context_parameters = WhisperContextParameters::default();
        context_parameters.use_gpu(config.use_gpu);
        let context = WhisperContext::new_with_params(model_path, context_parameters)
            .map_err(TranscriptionError::whisper)?;
        Ok(Self { context, config })
    }

    /// Transcribes mono 16 kHz floating-point samples entirely on-device.
    ///
    /// # Errors
    /// Returns an error if the input is empty or inference fails.
    pub fn transcribe(&self, audio: &MonoAudioBuffer) -> Result<Transcript, TranscriptionError> {
        self.transcribe_with_options(audio, self.config.language(), None)
    }

    /// Transcribes with a per-call language without reloading model weights.
    ///
    /// # Errors
    /// Returns an error if the input is empty or inference fails.
    pub fn transcribe_in_language(
        &self,
        audio: &MonoAudioBuffer,
        language: Option<&str>,
    ) -> Result<Transcript, TranscriptionError> {
        self.transcribe_with_options(audio, language, None)
    }

    /// Transcribes while allowing a preview pass to be cancelled before the
    /// final, latency-sensitive pass starts.
    ///
    /// # Errors
    /// Returns an error if the input is empty, inference fails, or cancellation
    /// is requested while Whisper is running.
    pub fn transcribe_cancellable(
        &self,
        audio: &MonoAudioBuffer,
        language: Option<&str>,
        abort: Arc<AtomicBool>,
    ) -> Result<Transcript, TranscriptionError> {
        self.transcribe_with_options(audio, language, Some(abort))
    }

    fn transcribe_with_options(
        &self,
        audio: &MonoAudioBuffer,
        language: Option<&str>,
        abort: Option<Arc<AtomicBool>>,
    ) -> Result<Transcript, TranscriptionError> {
        if audio.samples().is_empty() {
            return Err(TranscriptionError::new("cannot transcribe empty audio"));
        }
        let mut state = self
            .context
            .create_state()
            .map_err(TranscriptionError::whisper)?;
        let mut parameters = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        parameters.set_n_threads(self.config.thread_count);
        parameters.set_language(language.filter(|value| !value.trim().is_empty()));
        // A null language already asks whisper.cpp to auto-detect and then
        // continue decoding. Its separate detect_language flag means "detect
        // only" and returns before creating transcript segments.
        parameters.set_detect_language(false);
        parameters.set_translate(false);
        parameters.set_no_context(true);
        parameters.set_suppress_blank(true);
        parameters.set_suppress_nst(true);
        parameters.set_print_special(false);
        parameters.set_print_progress(false);
        parameters.set_print_realtime(false);
        parameters.set_print_timestamps(false);
        if let Some(abort) = abort {
            let callback: Box<dyn FnMut() -> bool> =
                Box::new(move || abort.load(Ordering::Acquire));
            parameters.set_abort_callback_safe::<_, Box<dyn FnMut() -> bool>>(Some(callback));
        }
        state
            .full(parameters, audio.samples())
            .map_err(TranscriptionError::whisper)?;
        let detected_language =
            whisper_rs::get_lang_str(state.full_lang_id_from_state()).map(str::to_owned);

        let mut segments = Vec::new();
        for segment in state.as_iter() {
            let text = segment
                .to_str_lossy()
                .map_err(TranscriptionError::whisper)?
                .trim()
                .to_owned();
            if !text.is_empty() {
                segments.push(TranscriptSegment {
                    start_centiseconds: segment.start_timestamp(),
                    end_centiseconds: segment.end_timestamp(),
                    text,
                    no_speech_probability: segment.no_speech_probability(),
                });
            }
        }
        let text = segments
            .iter()
            .map(|segment| segment.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        Ok(Transcript {
            text,
            segments,
            detected_language,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranscriptionError(String);

impl TranscriptionError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }

    fn whisper(error: impl fmt::Display) -> Self {
        Self::new(format!("whisper.cpp error: {error}"))
    }
}

impl fmt::Display for TranscriptionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for TranscriptionError {}

fn default_thread_count() -> i32 {
    let available = thread::available_parallelism().map_or(1, std::num::NonZero::get);
    i32::try_from(available).unwrap_or(i32::MAX).max(1)
}

#[cfg(test)]
mod tests {
    use super::{LocalSpeechServer, TranscriptionConfig, WhisperTranscriber};
    use std::path::Path;
    use std::{
        io::{Read, Write},
        net::{Ipv4Addr, TcpListener},
    };

    #[test]
    fn defaults_to_language_detection_and_bounded_parallelism() {
        let config = TranscriptionConfig::default();
        assert_eq!(config.language(), None);
        assert!(config.thread_count() >= 1);
        assert!(config.use_gpu());
        assert!(!config.with_gpu(false).use_gpu());
    }

    #[test]
    fn ignores_blank_fixed_language() {
        let config = TranscriptionConfig::default().with_language(Some("  ".to_owned()));
        assert_eq!(config.language(), None);
    }

    #[test]
    fn rejects_missing_model_before_entering_native_code() {
        let result = WhisperTranscriber::load(
            Path::new("definitely-missing-whisper-model.bin"),
            TranscriptionConfig::default(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn local_server_rejects_non_loopback_audio_destinations() {
        assert!(LocalSpeechServer::new("http://127.0.0.1:8080").is_ok());
        assert!(LocalSpeechServer::new("http://localhost").is_ok());
        assert!(LocalSpeechServer::new("http://LOCALHOST:8080").is_ok());
        assert!(LocalSpeechServer::new("http://[::1]:8080").is_ok());
        assert!(LocalSpeechServer::new("http://localhost:8080/v1/audio/transcriptions").is_ok());
        assert!(LocalSpeechServer::new("https://example.com").is_err());
        assert!(LocalSpeechServer::new("http://127.0.0.1.example.com").is_err());
        assert!(LocalSpeechServer::new("http://localhost:1@evil.example/").is_err());
        assert!(LocalSpeechServer::new("http://127.0.0.2:8080").is_ok());
    }

    #[test]
    fn local_server_sends_multipart_wav_and_reads_text() {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = Vec::new();
            let mut chunk = [0_u8; 4096];
            let expected = loop {
                let count = stream.read(&mut chunk).unwrap();
                request.extend_from_slice(&chunk[..count]);
                if let Some(header_end) = request.windows(4).position(|part| part == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request[..header_end]);
                    let length = headers
                        .lines()
                        .find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse::<usize>().ok())
                                .flatten()
                        })
                        .unwrap();
                    break header_end + 4 + length;
                }
            };
            while request.len() < expected {
                let count = stream.read(&mut chunk).unwrap();
                if count == 0 {
                    break;
                }
                request.extend_from_slice(&chunk[..count]);
            }
            assert!(request.windows(4).any(|part| part == b"RIFF"));
            assert!(String::from_utf8_lossy(&request).contains("name=\"language\"\r\n\r\nsv"));
            let body = r#"{"text":"hej världen","language":"sv"}"#;
            write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
        });
        let audio = spevox_audio::AudioBuffer::new(vec![0.0, 0.2, -0.2], 16_000, 1, false)
            .unwrap()
            .to_asr_mono();
        let transcript = LocalSpeechServer::new(&format!("http://{address}"))
            .unwrap()
            .transcribe(&audio, Some("sv"))
            .unwrap();
        assert_eq!(transcript.text, "hej världen");
        assert_eq!(transcript.detected_language.as_deref(), Some("sv"));
        server.join().unwrap();
    }
}
