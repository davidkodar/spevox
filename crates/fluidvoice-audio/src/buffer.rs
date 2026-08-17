use std::{error::Error, fmt, time::Duration};

pub const ASR_SAMPLE_RATE: u32 = 16_000;

/// Interleaved floating-point audio in the format negotiated with `PipeWire`.
#[derive(Clone, Debug, PartialEq)]
pub struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
    channels: u32,
    truncated: bool,
}

impl AudioBuffer {
    /// Constructs a validated interleaved audio buffer.
    ///
    /// # Errors
    ///
    /// Returns [`AudioFormatError`] for zero-valued format fields or when the
    /// sample count does not contain complete interleaved frames.
    pub fn new(
        samples: Vec<f32>,
        sample_rate: u32,
        channels: u32,
        truncated: bool,
    ) -> Result<Self, AudioFormatError> {
        if sample_rate == 0 {
            return Err(AudioFormatError::InvalidSampleRate);
        }
        if channels == 0 {
            return Err(AudioFormatError::InvalidChannelCount);
        }
        let channel_count =
            usize::try_from(channels).map_err(|_| AudioFormatError::InvalidChannelCount)?;
        if !samples.len().is_multiple_of(channel_count) {
            return Err(AudioFormatError::IncompleteFrame {
                samples: samples.len(),
                channels,
            });
        }

        Ok(Self {
            samples,
            sample_rate,
            channels,
            truncated,
        })
    }

    #[must_use]
    pub fn samples(&self) -> &[f32] {
        &self.samples
    }

    #[must_use]
    pub const fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[must_use]
    pub const fn channels(&self) -> u32 {
        self.channels
    }

    #[must_use]
    pub const fn truncated(&self) -> bool {
        self.truncated
    }

    #[must_use]
    pub fn frame_count(&self) -> usize {
        self.samples.len() / usize::try_from(self.channels).unwrap_or(1)
    }

    #[must_use]
    pub fn duration(&self) -> Duration {
        duration_for_frames(self.frame_count(), self.sample_rate)
    }

    #[must_use]
    pub fn peak(&self) -> f32 {
        self.samples
            .iter()
            .copied()
            .map(f32::abs)
            .fold(0.0, f32::max)
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn rms(&self) -> f32 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let sum = self
            .samples
            .iter()
            .copied()
            .map(|sample| sample * sample)
            .sum::<f32>();
        (sum / self.samples.len() as f32).sqrt()
    }

    /// Downmixes and band-limited resamples to the mono 16 kHz ASR contract.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_asr_mono(&self) -> MonoAudioBuffer {
        let channels = usize::try_from(self.channels).unwrap_or(1);
        let mut mono = Vec::with_capacity(self.frame_count());
        for frame in self.samples.chunks_exact(channels) {
            mono.push(frame.iter().copied().sum::<f32>() / self.channels as f32);
        }

        if self.sample_rate == ASR_SAMPLE_RATE || mono.is_empty() {
            return MonoAudioBuffer { samples: mono };
        }

        MonoAudioBuffer {
            samples: band_limited_resample(&mono, self.sample_rate, ASR_SAMPLE_RATE),
        }
    }
}

/// Preserves resampling phase between realtime capture chunks.
///
/// A short look-ahead is intentionally retained, so the final few input
/// samples are omitted from previews; the authoritative full-utterance pass
/// still uses [`AudioBuffer::to_asr_mono`].
#[derive(Default)]
pub struct StreamingAsrConverter {
    samples: Vec<f32>,
    input_rate: u32,
    channels: u32,
    next_output: usize,
    kernel: Option<SincKernel>,
}

impl StreamingAsrConverter {
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn process(&mut self, input: &AudioBuffer) -> MonoAudioBuffer {
        if self.input_rate != input.sample_rate || self.channels != input.channels {
            self.samples.clear();
            self.next_output = 0;
            self.input_rate = input.sample_rate;
            self.channels = input.channels;
            self.kernel = (self.input_rate != ASR_SAMPLE_RATE)
                .then(|| SincKernel::new(self.input_rate, ASR_SAMPLE_RATE));
        }
        let channels = usize::try_from(input.channels).unwrap_or(1);
        self.samples.extend(
            input
                .samples
                .chunks_exact(channels)
                .map(|frame| frame.iter().copied().sum::<f32>() / input.channels as f32),
        );
        if self.input_rate == ASR_SAMPLE_RATE {
            let output = self.samples[self.next_output..].to_vec();
            self.next_output = self.samples.len();
            return MonoAudioBuffer { samples: output };
        }

        let available_outputs = resampled_length(
            self.samples.len().saturating_sub(SINC_HALF_TAPS),
            self.input_rate,
            ASR_SAMPLE_RATE,
        );
        let Some(kernel) = self.kernel.as_ref() else {
            return MonoAudioBuffer {
                samples: Vec::new(),
            };
        };
        let output = (self.next_output..available_outputs)
            .map(|index| kernel.sample(&self.samples, index))
            .collect();
        self.next_output = available_outputs;
        MonoAudioBuffer { samples: output }
    }
}

/// Mono 16 kHz floating-point samples accepted by transcription providers.
#[derive(Clone, Debug, PartialEq)]
pub struct MonoAudioBuffer {
    samples: Vec<f32>,
}

impl MonoAudioBuffer {
    #[must_use]
    pub fn samples(&self) -> &[f32] {
        &self.samples
    }

    #[must_use]
    pub const fn sample_rate(&self) -> u32 {
        ASR_SAMPLE_RATE
    }

    #[must_use]
    pub fn duration(&self) -> Duration {
        duration_for_frames(self.samples.len(), ASR_SAMPLE_RATE)
    }

    #[must_use]
    pub fn peak(&self) -> f32 {
        self.samples
            .iter()
            .copied()
            .map(f32::abs)
            .fold(0.0, f32::max)
    }

    /// Returns a copy with finite samples amplified and clamped to the ASR range.
    #[must_use]
    pub fn amplified(&self, gain: f32) -> Self {
        let gain = if gain.is_finite() {
            gain.clamp(0.0, 64.0)
        } else {
            1.0
        };
        Self {
            samples: self
                .samples
                .iter()
                .map(|sample| (sample * gain).clamp(-1.0, 1.0))
                .collect(),
        }
    }

    /// Removes quiet leading and trailing windows while retaining a short
    /// cushion around speech. Interior pauses are intentionally preserved.
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // 20 ms windows are always exactly 320 samples.
    pub fn trim_silence(&self) -> Self {
        const WINDOW: usize = 320; // 20 ms at 16 kHz
        const PADDING_WINDOWS: usize = 10; // 200 ms
        if self.samples.len() < WINDOW * 2 {
            return self.clone();
        }

        let peak = self.peak();
        let threshold = (peak * 0.035).clamp(0.000_5, 0.025);
        let active = |window: &[f32]| {
            let energy =
                window.iter().map(|sample| sample * sample).sum::<f32>() / window.len() as f32;
            energy.sqrt() >= threshold
        };
        let first = self.samples.chunks(WINDOW).position(active);
        let last = self.samples.chunks(WINDOW).rposition(active);
        let (Some(first), Some(last)) = (first, last) else {
            return Self {
                samples: Vec::new(),
            };
        };
        let start = first.saturating_sub(PADDING_WINDOWS) * WINDOW;
        let end = ((last + PADDING_WINDOWS + 1) * WINDOW).min(self.samples.len());
        Self {
            samples: self.samples[start..end].to_vec(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioFormatError {
    InvalidSampleRate,
    InvalidChannelCount,
    IncompleteFrame { samples: usize, channels: u32 },
}

impl fmt::Display for AudioFormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSampleRate => formatter.write_str("sample rate must be greater than zero"),
            Self::InvalidChannelCount => {
                formatter.write_str("channel count must be greater than zero")
            }
            Self::IncompleteFrame { samples, channels } => write!(
                formatter,
                "{samples} samples do not contain complete {channels}-channel frames"
            ),
        }
    }
}

impl Error for AudioFormatError {}

#[allow(clippy::cast_precision_loss)]
fn duration_for_frames(frames: usize, sample_rate: u32) -> Duration {
    let frame_count = u64::try_from(frames).unwrap_or(u64::MAX);
    Duration::from_secs_f64(frame_count as f64 / f64::from(sample_rate))
}

fn resampled_length(input_length: usize, input_rate: u32, output_rate: u32) -> usize {
    let input = u128::try_from(input_length).unwrap_or(u128::MAX);
    let rounded = input
        .saturating_mul(u128::from(output_rate))
        .saturating_add(u128::from(input_rate / 2))
        / u128::from(input_rate);
    usize::try_from(rounded).unwrap_or(usize::MAX)
}

const SINC_HALF_TAPS: usize = 12;

#[allow(clippy::cast_precision_loss)]
fn band_limited_resample(input: &[f32], input_rate: u32, output_rate: u32) -> Vec<f32> {
    let kernel = SincKernel::new(input_rate, output_rate);
    (0..resampled_length(input.len(), input_rate, output_rate))
        .map(|index| kernel.sample(input, index))
        .collect()
}

struct SincKernel {
    input_rate: u32,
    output_rate: u32,
    phase_divisor: u32,
    phases: Vec<Vec<f32>>,
}

impl SincKernel {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_precision_loss
    )]
    fn new(input_rate: u32, output_rate: u32) -> Self {
        let divisor = greatest_common_divisor(input_rate, output_rate);
        let phase_count = output_rate / divisor;
        let cutoff = (f64::from(output_rate) / f64::from(input_rate)).min(1.0) * 0.92;
        let phases = (0..phase_count)
            .map(|phase| {
                let fraction = f64::from(phase) / f64::from(phase_count);
                (-(SINC_HALF_TAPS as isize)..=(SINC_HALF_TAPS as isize))
                    .map(|tap| {
                        let distance = fraction - tap as f64;
                        let argument = std::f64::consts::PI * cutoff * distance;
                        let sinc = if argument.abs() < f64::EPSILON {
                            1.0
                        } else {
                            argument.sin() / argument
                        };
                        let window_position = distance / (SINC_HALF_TAPS as f64 + 1.0);
                        let window = 0.5 * (1.0 + (std::f64::consts::PI * window_position).cos());
                        (cutoff * sinc * window) as f32
                    })
                    .collect()
            })
            .collect();
        Self {
            input_rate,
            output_rate,
            phase_divisor: divisor,
            phases,
        }
    }

    fn sample(&self, input: &[f32], output_index: usize) -> f32 {
        let numerator =
            u64::try_from(output_index).unwrap_or(u64::MAX) * u64::from(self.input_rate);
        let center = isize::try_from(numerator / u64::from(self.output_rate)).unwrap_or(isize::MAX);
        let remainder = u32::try_from(numerator % u64::from(self.output_rate)).unwrap_or_default();
        let phase = usize::try_from(remainder / self.phase_divisor).unwrap_or_default();
        let mut weighted = 0.0_f32;
        let mut weight_sum = 0.0_f32;
        for (offset, weight) in self.phases[phase].iter().enumerate() {
            let tap = isize::try_from(offset).unwrap_or(isize::MAX)
                - isize::try_from(SINC_HALF_TAPS).unwrap_or_default();
            let Ok(source) = usize::try_from(center.saturating_add(tap)) else {
                continue;
            };
            let Some(sample) = input.get(source) else {
                continue;
            };
            weighted += sample * weight;
            weight_sum += weight;
        }
        if weight_sum.abs() < f32::EPSILON {
            0.0
        } else {
            weighted / weight_sum
        }
    }
}

const fn greatest_common_divisor(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

#[cfg(test)]
#[allow(clippy::cast_precision_loss)]
mod tests {
    use super::{ASR_SAMPLE_RATE, AudioBuffer, AudioFormatError, StreamingAsrConverter};

    #[test]
    fn rejects_incomplete_interleaved_frame() {
        let error = AudioBuffer::new(vec![0.0, 1.0, 2.0], 48_000, 2, false).unwrap_err();
        assert_eq!(
            error,
            AudioFormatError::IncompleteFrame {
                samples: 3,
                channels: 2,
            }
        );
    }

    #[test]
    fn downmixes_stereo_by_averaging_channels() {
        let input = AudioBuffer::new(vec![1.0, -1.0, 0.5, 0.5], ASR_SAMPLE_RATE, 2, false).unwrap();
        let output = input.to_asr_mono();

        assert_eq!(output.samples(), &[0.0, 0.5]);
    }

    #[test]
    fn resamples_48khz_to_16khz() {
        let samples = vec![0.0; 48_000];
        let input = AudioBuffer::new(samples, 48_000, 1, false).unwrap();
        let output = input.to_asr_mono();

        assert_eq!(output.sample_rate(), ASR_SAMPLE_RATE);
        assert_eq!(output.samples().len(), 16_000);
        assert_eq!(output.duration(), std::time::Duration::from_secs(1));
    }

    #[test]
    fn suppresses_frequencies_above_the_asr_nyquist_limit() {
        let samples = (0..48_000)
            .map(|index| (2.0 * std::f32::consts::PI * 12_000.0 * index as f32 / 48_000.0).sin())
            .collect();
        let output = AudioBuffer::new(samples, 48_000, 1, false)
            .unwrap()
            .to_asr_mono();
        let rms = (output
            .samples()
            .iter()
            .map(|value| value * value)
            .sum::<f32>()
            / output.samples().len() as f32)
            .sqrt();
        assert!(rms < 0.08, "aliased out-of-band tone RMS was {rms}");
    }

    #[test]
    fn streaming_conversion_matches_one_shot_phase() {
        let samples = (0..4_800)
            .map(|index| (index as f32 * 0.031).sin())
            .collect::<Vec<_>>();
        let full = AudioBuffer::new(samples.clone(), 48_000, 1, false)
            .unwrap()
            .to_asr_mono();
        let mut converter = StreamingAsrConverter::default();
        let mut streamed = Vec::new();
        for chunk in samples.chunks(777) {
            streamed.extend(
                converter
                    .process(&AudioBuffer::new(chunk.to_vec(), 48_000, 1, false).unwrap())
                    .samples()
                    .iter()
                    .copied(),
            );
        }
        assert!(streamed.len() > 1_500);
        for (actual, expected) in streamed.iter().zip(full.samples()) {
            assert!((actual - expected).abs() < 1.0e-5);
        }
    }

    #[test]
    fn streaming_44khz_conversion_preserves_phase_across_uneven_chunks() {
        let samples = (0..44_100)
            .map(|index| (index as f32 * 0.017).sin())
            .collect::<Vec<_>>();
        let full = AudioBuffer::new(samples.clone(), 44_100, 1, false)
            .unwrap()
            .to_asr_mono();
        let mut converter = StreamingAsrConverter::default();
        let mut streamed = Vec::new();
        for chunk in samples.chunks(613) {
            streamed.extend(
                converter
                    .process(&AudioBuffer::new(chunk.to_vec(), 44_100, 1, false).unwrap())
                    .samples()
                    .iter()
                    .copied(),
            );
        }
        assert!(streamed.len() > 15_900);
        for (actual, expected) in streamed.iter().zip(full.samples()) {
            assert!((actual - expected).abs() < 1.0e-5);
        }
    }

    #[test]
    fn reports_peak_and_rms() {
        let input = AudioBuffer::new(vec![-1.0, 1.0, -1.0, 1.0], 16_000, 1, false).unwrap();

        assert!((input.peak() - 1.0).abs() < f32::EPSILON);
        assert!((input.rms() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn amplifies_and_clamps_mono_audio() {
        let audio = AudioBuffer::new(vec![0.1, -0.75], ASR_SAMPLE_RATE, 1, false)
            .unwrap()
            .to_asr_mono();
        assert_eq!(audio.amplified(2.0).samples(), &[0.2, -1.0]);
    }

    #[test]
    fn trims_only_outer_silence_and_keeps_padding() {
        let mut samples = vec![0.0; ASR_SAMPLE_RATE as usize];
        samples.extend(vec![0.2; ASR_SAMPLE_RATE as usize / 2]);
        samples.extend(vec![0.0; ASR_SAMPLE_RATE as usize]);
        let audio = AudioBuffer::new(samples, ASR_SAMPLE_RATE, 1, false)
            .unwrap()
            .to_asr_mono()
            .trim_silence();
        assert!(audio.duration() >= std::time::Duration::from_millis(850));
        assert!(audio.duration() <= std::time::Duration::from_millis(950));
    }

    #[test]
    fn silence_trims_to_empty() {
        let audio = AudioBuffer::new(vec![0.0; 16_000], ASR_SAMPLE_RATE, 1, false)
            .unwrap()
            .to_asr_mono()
            .trim_silence();
        assert!(audio.samples().is_empty());
    }
}
