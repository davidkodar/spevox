//! `PipeWire` microphone capture and ASR boundary conversion.

mod buffer;
mod capture;
mod wav;

pub use buffer::{AudioBuffer, AudioFormatError, MonoAudioBuffer, StreamingAsrConverter};
pub use capture::{AudioCaptureError, AudioDevice, CaptureStopToken, PipeWireCapture};
pub use wav::{WavEncodeError, encode_pcm16_wav, encode_samples, pcm_i16};
