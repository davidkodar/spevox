//! Offline speech-to-text through `whisper.cpp`.

use std::{error::Error, fmt, path::Path, thread};

use fluidvoice_audio::MonoAudioBuffer;
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
        if audio.samples().is_empty() {
            return Err(TranscriptionError::new("cannot transcribe empty audio"));
        }
        let mut state = self
            .context
            .create_state()
            .map_err(TranscriptionError::whisper)?;
        let mut parameters = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        parameters.set_n_threads(self.config.thread_count);
        parameters.set_language(self.config.language());
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
    i32::try_from(available.min(8)).unwrap_or(8)
}

#[cfg(test)]
mod tests {
    use super::{TranscriptionConfig, WhisperTranscriber};
    use std::path::Path;

    #[test]
    fn defaults_to_language_detection_and_bounded_parallelism() {
        let config = TranscriptionConfig::default();
        assert_eq!(config.language(), None);
        assert!((1..=8).contains(&config.thread_count()));
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
}
