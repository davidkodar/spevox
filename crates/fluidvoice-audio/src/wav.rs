use std::{error::Error, fmt};

use crate::MonoAudioBuffer;

/// Encodes mono floating-point audio as a standard PCM16 WAV byte stream.
///
/// # Errors
/// Returns an error when the sample payload cannot fit in the WAV/RF64-free
/// 32-bit chunk format.
pub fn encode_pcm16_wav(audio: &MonoAudioBuffer) -> Result<Vec<u8>, WavEncodeError> {
    encode_samples(audio.samples(), audio.sample_rate())
}

/// Encodes mono floating-point samples as a standard PCM16 WAV byte stream.
///
/// # Errors
/// Returns an error when the sample payload cannot fit in the WAV/RF64-free
/// 32-bit chunk format.
pub fn encode_samples(samples: &[f32], sample_rate: u32) -> Result<Vec<u8>, WavEncodeError> {
    let data_size = samples
        .len()
        .checked_mul(2)
        .and_then(|size| u32::try_from(size).ok())
        .ok_or(WavEncodeError)?;
    let riff_size = 36_u32.checked_add(data_size).ok_or(WavEncodeError)?;
    let byte_rate = sample_rate.checked_mul(2).ok_or(WavEncodeError)?;
    let capacity = 44_usize
        .checked_add(usize::try_from(data_size).map_err(|_| WavEncodeError)?)
        .ok_or(WavEncodeError)?;
    let mut wav = Vec::with_capacity(capacity);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&riff_size.to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    for &sample in samples {
        wav.extend_from_slice(&pcm_i16(sample).to_le_bytes());
    }
    Ok(wav)
}

/// Clamps, rounds, and converts one normalized sample to signed PCM16.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn pcm_i16(sample: f32) -> i16 {
    (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WavEncodeError;

impl fmt::Display for WavEncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("audio is too large for a standard PCM WAV file")
    }
}

impl Error for WavEncodeError {}

#[cfg(test)]
mod tests {
    use super::encode_samples;

    #[test]
    fn encodes_standard_mono_pcm16_wav() {
        let wav = encode_samples(&[-1.0, 0.0, 1.0], 16_000).unwrap();
        assert_eq!(&wav[..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(u32::from_le_bytes(wav[24..28].try_into().unwrap()), 16_000);
        assert_eq!(u32::from_le_bytes(wav[40..44].try_into().unwrap()), 6);
        assert_eq!(i16::from_le_bytes(wav[44..46].try_into().unwrap()), -32_767);
        assert_eq!(i16::from_le_bytes(wav[48..50].try_into().unwrap()), 32_767);
    }
}
