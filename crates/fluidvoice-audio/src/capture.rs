use std::{
    cell::{Cell, RefCell},
    error::Error,
    fmt,
    io::Cursor,
    mem,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use pipewire as pw;
use pw::{properties::properties, spa};
use spa::{
    param::audio::{AudioFormat, AudioInfoRaw},
    param::format::{MediaSubtype, MediaType},
    param::format_utils,
    pod::{Pod, Value},
};

use crate::{AudioBuffer, AudioFormatError};

const MAX_DURATION: Duration = Duration::from_mins(2);
const MAX_SAMPLE_RATE: u128 = 192_000;
const MAX_CHANNELS: u128 = 8;
const PREVIEW_WINDOW_SECONDS: usize = 15;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AudioDevice {
    pub id: u32,
    pub node_name: String,
    pub description: String,
}

pub struct PipeWireCapture;

/// Thread-safe signal used to end an active microphone capture.
#[derive(Clone, Debug, Default)]
pub struct CaptureStopToken(Arc<AtomicBool>);

impl CaptureStopToken {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stop(&self) {
        self.0.store(true, Ordering::Release);
    }

    #[must_use]
    pub fn is_stopped(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}

impl PipeWireCapture {
    /// Lists microphone sources currently exposed by `PipeWire`.
    ///
    /// # Errors
    /// Returns an error when the `PipeWire` server cannot be reached or synced.
    pub fn devices() -> Result<Vec<AudioDevice>, AudioCaptureError> {
        pw::init();
        let main_loop = pw::main_loop::MainLoopRc::new(None).map_err(AudioCaptureError::pw)?;
        let context =
            pw::context::ContextRc::new(&main_loop, None).map_err(AudioCaptureError::pw)?;
        let core = context.connect_rc(None).map_err(AudioCaptureError::pw)?;
        let registry = core.get_registry().map_err(AudioCaptureError::pw)?;
        let devices = Rc::new(RefCell::new(Vec::new()));
        let output = Rc::clone(&devices);
        let _registry_listener = registry
            .add_listener_local()
            .global(move |global| {
                if global.type_ != pw::types::ObjectType::Node {
                    return;
                }
                let Some(props) = global.props else { return };
                if props.get(*pw::keys::MEDIA_CLASS) != Some("Audio/Source") {
                    return;
                }
                let Some(name) = props.get(*pw::keys::NODE_NAME) else {
                    return;
                };
                output.borrow_mut().push(AudioDevice {
                    id: global.id,
                    node_name: name.to_owned(),
                    description: props
                        .get(*pw::keys::NODE_DESCRIPTION)
                        .unwrap_or(name)
                        .to_owned(),
                });
            })
            .register();
        let pending = core.sync(0).map_err(AudioCaptureError::pw)?;
        let completed = Rc::new(Cell::new(false));
        let completed_signal = Rc::clone(&completed);
        let discovery_error = Rc::new(RefCell::new(None::<String>));
        let core_error = Rc::clone(&discovery_error);
        let loop_clone = main_loop.clone();
        let error_loop = main_loop.clone();
        let _core_listener = core
            .add_listener_local()
            .done(move |id, sequence| {
                if id == pw::core::PW_ID_CORE && sequence == pending {
                    completed_signal.set(true);
                    loop_clone.quit();
                }
            })
            .error(move |_, _, _, message| {
                *core_error.borrow_mut() = Some(message.to_owned());
                error_loop.quit();
            })
            .register();
        let timeout_loop = main_loop.clone();
        let timer = main_loop.loop_().add_timer(move |_| timeout_loop.quit());
        timer
            .update_timer(Some(Duration::from_secs(3)), None)
            .into_result()
            .map_err(AudioCaptureError::pw)?;
        main_loop.run();
        if let Some(error) = discovery_error.borrow().as_ref() {
            return Err(AudioCaptureError::new(format!(
                "PipeWire device discovery failed: {error}"
            )));
        }
        if !completed.get() {
            return Err(AudioCaptureError::new(
                "PipeWire device discovery timed out after 3 seconds",
            ));
        }
        let mut result = devices.borrow().clone();
        result.sort_by(|a, b| a.description.cmp(&b.description));
        result.dedup_by(|a, b| a.node_name == b.node_name);
        Ok(result)
    }

    /// Captures native F32LE microphone audio for a bounded duration.
    ///
    /// # Errors
    /// Returns an error for invalid durations, `PipeWire` failures, or empty audio.
    pub fn capture_for(
        duration: Duration,
        target: Option<&str>,
    ) -> Result<AudioBuffer, AudioCaptureError> {
        Self::capture_until_stopped(duration, target, &CaptureStopToken::new())
    }

    /// Captures until `stop_token` is signalled, with a maximum safety duration.
    ///
    /// The token may be signalled from the portal event task while `PipeWire` is
    /// running on its dedicated blocking thread.
    ///
    /// # Errors
    /// Returns an error for invalid durations, `PipeWire` failures, or empty audio.
    pub fn capture_until_stopped(
        maximum_duration: Duration,
        target: Option<&str>,
        stop_token: &CaptureStopToken,
    ) -> Result<AudioBuffer, AudioCaptureError> {
        Self::capture_with_levels(maximum_duration, target, stop_token, |_| {})
    }

    /// Captures until stopped and reports the peak of each received audio chunk.
    ///
    /// # Errors
    /// Returns an error for invalid durations, `PipeWire` failures, or empty audio.
    #[allow(clippy::too_many_lines)]
    pub fn capture_with_levels(
        maximum_duration: Duration,
        target: Option<&str>,
        stop_token: &CaptureStopToken,
        report_level: impl FnMut(f32) + 'static,
    ) -> Result<AudioBuffer, AudioCaptureError> {
        Self::capture_with_preview(maximum_duration, target, stop_token, report_level, |_| {})
    }

    /// Captures until stopped, reporting levels and periodic accumulated-audio
    /// snapshots suitable for live transcription previews.
    ///
    /// # Errors
    /// Returns an error for invalid durations, `PipeWire` failures, or empty audio.
    #[allow(clippy::too_many_lines)]
    pub fn capture_with_preview(
        maximum_duration: Duration,
        target: Option<&str>,
        stop_token: &CaptureStopToken,
        report_level: impl FnMut(f32) + 'static,
        report_preview: impl FnMut(AudioBuffer) + 'static,
    ) -> Result<AudioBuffer, AudioCaptureError> {
        Self::capture_with_streaming_preview(
            maximum_duration,
            target,
            stop_token,
            report_level,
            report_preview,
            |_| {},
        )
    }

    /// Captures audio while additionally reporting every newly received chunk.
    /// The chunk callback is intended for realtime speech engines; preview
    /// snapshots remain bounded and suitable for slower batch decoders.
    ///
    /// # Errors
    /// Returns an error for invalid durations, `PipeWire` failures, or empty audio.
    #[allow(clippy::too_many_lines)]
    pub fn capture_with_streaming_preview(
        maximum_duration: Duration,
        target: Option<&str>,
        stop_token: &CaptureStopToken,
        mut report_level: impl FnMut(f32) + 'static,
        mut report_preview: impl FnMut(AudioBuffer) + 'static,
        mut report_stream_chunk: impl FnMut(AudioBuffer) + 'static,
    ) -> Result<AudioBuffer, AudioCaptureError> {
        validate_duration(maximum_duration)?;
        let capacity = capture_capacity(maximum_duration)?;
        pw::init();
        let main_loop = pw::main_loop::MainLoopRc::new(None).map_err(AudioCaptureError::pw)?;
        let context =
            pw::context::ContextRc::new(&main_loop, None).map_err(AudioCaptureError::pw)?;
        let core = context.connect_rc(None).map_err(AudioCaptureError::pw)?;
        let mut props = properties! {
            *pw::keys::MEDIA_TYPE => "Audio",
            *pw::keys::MEDIA_CATEGORY => "Capture",
            *pw::keys::MEDIA_ROLE => "Communication",
            *pw::keys::APP_ID => "io.github.davidkodar.FluidVoiceLinux",
            *pw::keys::NODE_NAME => "fluidvoice-linux-capture",
            *pw::keys::NODE_DESCRIPTION => "FluidVoice Linux microphone capture",
        };
        if let Some(target) = target {
            props.insert(*pw::keys::TARGET_OBJECT, target);
        }
        let stream = pw::stream::StreamBox::new(&core, "fluidvoice-linux-capture", props)
            .map_err(AudioCaptureError::pw)?;
        let captured = Rc::new(RefCell::new(CapturedSamples::new(capacity)));
        let format_output = Rc::clone(&captured);
        let sample_output = Rc::clone(&captured);
        let capture_error = Rc::new(RefCell::new(None::<String>));
        let stream_error = Rc::clone(&capture_error);
        let error_loop = main_loop.clone();
        let mut last_preview = Instant::now();
        let _listener = stream
            .add_local_listener_with_user_data(CaptureData::default())
            .state_changed(move |_, _, _, state| {
                if let pw::stream::StreamState::Error(message) = state {
                    *stream_error.borrow_mut() = Some(message);
                    error_loop.quit();
                }
            })
            .param_changed(move |_, data, id, param| {
                let Some(param) = param else { return };
                if id != spa::param::ParamType::Format.as_raw()
                    || format_utils::parse_format(param).ok()
                        != Some((MediaType::Audio, MediaSubtype::Raw))
                {
                    return;
                }
                if data.format.parse(param).is_ok() {
                    let mut output = format_output.borrow_mut();
                    output.sample_rate = data.format.rate();
                    output.channels = data.format.channels();
                }
            })
            .process(move |stream, _| {
                let Some(mut buffer) = stream.dequeue_buffer() else {
                    return;
                };
                let Some(data) = buffer.datas_mut().first_mut() else {
                    return;
                };
                let chunk = data.chunk();
                let offset = usize::try_from(chunk.offset()).unwrap_or(usize::MAX);
                let size = usize::try_from(chunk.size()).unwrap_or(usize::MAX);
                if let Some(bytes) = data.data() {
                    let before = sample_output.borrow().samples.len();
                    let peak = sample_output.borrow_mut().append_chunk(bytes, offset, size);
                    report_level(peak);
                    let output = sample_output.borrow();
                    let channels = usize::try_from(output.channels).unwrap_or(1);
                    let after = output.samples.len();
                    if output.sample_rate > 0 && after > before {
                        let start = before - before % channels;
                        let complete = after - after % channels;
                        if complete > start
                            && let Ok(chunk) = AudioBuffer::new(
                                output.samples[start..complete].to_vec(),
                                output.sample_rate,
                                output.channels,
                                output.truncated,
                            )
                        {
                            report_stream_chunk(chunk);
                        }
                    }
                    drop(output);
                    // FluidVoice's non-native streaming engines refresh near
                    // 0.6 seconds. Slow models naturally coalesce updates via
                    // the bounded channel in the UI layer.
                    if last_preview.elapsed() >= Duration::from_millis(650) {
                        let output = sample_output.borrow();
                        if output.sample_rate > 0
                            && output.channels > 0
                            && !output.samples.is_empty()
                        {
                            let channels = usize::try_from(output.channels).unwrap_or(1);
                            let maximum_samples = usize::try_from(output.sample_rate)
                                .unwrap_or_default()
                                .saturating_mul(channels)
                                .saturating_mul(PREVIEW_WINDOW_SECONDS);
                            let complete = output.samples.len() - output.samples.len() % channels;
                            let start = complete.saturating_sub(maximum_samples);
                            if let Ok(snapshot) = AudioBuffer::new(
                                output.samples[start..complete].to_vec(),
                                output.sample_rate,
                                output.channels,
                                output.truncated,
                            ) {
                                report_preview(snapshot);
                            }
                        }
                        last_preview = Instant::now();
                    }
                }
            })
            .register()
            .map_err(AudioCaptureError::pw)?;

        let mut info = AudioInfoRaw::new();
        info.set_format(AudioFormat::F32LE);
        let object = spa::pod::Object {
            type_: spa::utils::SpaTypes::ObjectParamFormat.as_raw(),
            id: spa::param::ParamType::EnumFormat.as_raw(),
            properties: info.into(),
        };
        let bytes = spa::pod::serialize::PodSerializer::serialize(
            Cursor::new(Vec::new()),
            &Value::Object(object),
        )
        .map_err(|e| AudioCaptureError::new(format!("audio format serialization failed: {e}")))?
        .0
        .into_inner();
        let param = Pod::from_bytes(&bytes)
            .ok_or_else(|| AudioCaptureError::new("invalid audio format pod"))?;
        stream
            .connect(
                spa::utils::Direction::Input,
                None,
                pw::stream::StreamFlags::AUTOCONNECT | pw::stream::StreamFlags::MAP_BUFFERS,
                &mut [param],
            )
            .map_err(AudioCaptureError::pw)?;
        let loop_clone = main_loop.clone();
        let stop_token = stop_token.clone();
        let started = Instant::now();
        let timer = main_loop.loop_().add_timer(move |_| {
            if stop_token.is_stopped() || started.elapsed() >= maximum_duration {
                loop_clone.quit();
            }
        });
        let poll_interval = Duration::from_millis(10);
        timer
            .update_timer(Some(poll_interval), Some(poll_interval))
            .into_result()
            .map_err(AudioCaptureError::pw)?;
        main_loop.run();
        stream.disconnect().map_err(AudioCaptureError::pw)?;

        if let Some(error) = capture_error.borrow().as_ref() {
            return Err(AudioCaptureError::new(format!(
                "PipeWire capture stream failed: {error}"
            )));
        }

        let mut result = captured.borrow_mut();
        if result.sample_rate == 0 || result.channels == 0 {
            return Err(AudioCaptureError::new("no raw-audio format was negotiated"));
        }
        if result.samples.is_empty() {
            return Err(AudioCaptureError::new(
                "no microphone samples were received",
            ));
        }
        let channels = usize::try_from(result.channels)
            .map_err(|_| AudioCaptureError::new("invalid channel count"))?;
        let complete = result.samples.len() - result.samples.len() % channels;
        result.samples.truncate(complete);
        AudioBuffer::new(
            mem::take(&mut result.samples),
            result.sample_rate,
            result.channels,
            result.truncated,
        )
        .map_err(AudioCaptureError::format)
    }
}

#[derive(Default)]
struct CaptureData {
    format: AudioInfoRaw,
}

struct CapturedSamples {
    samples: Vec<f32>,
    maximum_samples: usize,
    sample_rate: u32,
    channels: u32,
    truncated: bool,
}

impl CapturedSamples {
    fn new(capacity: usize) -> Self {
        // Reserve a modest working set and let the vector grow only when the
        // negotiated format requires it. The previous worst-case reservation
        // could request hundreds of MiB before PipeWire supplied a format.
        let initial_capacity = capacity.min(48_000 * 2 * PREVIEW_WINDOW_SECONDS);
        Self {
            samples: Vec::with_capacity(initial_capacity),
            maximum_samples: capacity,
            sample_rate: 0,
            channels: 0,
            truncated: false,
        }
    }

    fn append(&mut self, bytes: &[u8]) -> f32 {
        let mut peak = 0.0_f32;
        for chunk in bytes.chunks_exact(mem::size_of::<f32>()) {
            if self.samples.len() == self.maximum_samples {
                self.truncated = true;
                break;
            }
            let decoded = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let sample = if decoded.is_finite() { decoded } else { 0.0 };
            peak = peak.max(sample.abs());
            self.samples.push(sample);
        }
        peak
    }

    fn append_chunk(&mut self, bytes: &[u8], offset: usize, size: usize) -> f32 {
        let Some(available) = bytes.get(offset..) else {
            return 0.0;
        };
        self.append(&available[..size.min(available.len())])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AudioCaptureError(String);

impl AudioCaptureError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
    fn pw(error: impl fmt::Display) -> Self {
        Self::new(format!("PipeWire error: {error}"))
    }
    fn format(error: AudioFormatError) -> Self {
        Self::new(format!("invalid audio: {error}"))
    }
}

impl fmt::Display for AudioCaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl Error for AudioCaptureError {}

fn validate_duration(duration: Duration) -> Result<(), AudioCaptureError> {
    if duration.is_zero() || duration > MAX_DURATION {
        return Err(AudioCaptureError::new(
            "capture duration must be greater than zero and at most 120 seconds",
        ));
    }
    Ok(())
}

fn capture_capacity(duration: Duration) -> Result<usize, AudioCaptureError> {
    let samples = duration
        .as_nanos()
        .saturating_mul(MAX_SAMPLE_RATE)
        .saturating_mul(MAX_CHANNELS)
        .div_ceil(1_000_000_000);
    usize::try_from(samples).map_err(|_| AudioCaptureError::new("capture is too large"))
}

#[cfg(test)]
mod tests {
    use super::{CaptureStopToken, CapturedSamples, capture_capacity, validate_duration};
    use std::time::Duration;

    #[test]
    fn rejects_zero_duration() {
        assert!(validate_duration(Duration::ZERO).is_err());
    }

    #[test]
    fn stop_token_is_shared_across_threads() {
        let token = CaptureStopToken::new();
        let callback_token = token.clone();
        assert!(!callback_token.is_stopped());
        token.stop();
        assert!(callback_token.is_stopped());
    }

    #[test]
    fn preallocates_worst_case() {
        assert_eq!(capture_capacity(Duration::from_secs(3)).unwrap(), 4_608_000);
    }

    #[test]
    fn decodes_and_caps_samples() {
        let mut output = CapturedSamples::new(2);
        let bytes = [1.0_f32, -0.5, 0.25]
            .into_iter()
            .flat_map(f32::to_le_bytes)
            .collect::<Vec<_>>();
        output.append(&bytes);
        assert_eq!(output.samples, vec![1.0, -0.5]);
        assert!(output.truncated);
    }

    #[test]
    fn honors_pipewire_chunk_offset_and_size() {
        let mut output = CapturedSamples::new(4);
        let bytes = [99.0_f32, 1.0, -0.5, 88.0]
            .into_iter()
            .flat_map(f32::to_le_bytes)
            .collect::<Vec<_>>();
        output.append_chunk(&bytes, 4, 8);
        assert_eq!(output.samples, vec![1.0, -0.5]);
    }

    #[test]
    fn replaces_non_finite_samples_with_silence() {
        let mut output = CapturedSamples::new(3);
        let bytes = [f32::NAN, f32::INFINITY, 0.25]
            .into_iter()
            .flat_map(f32::to_le_bytes)
            .collect::<Vec<_>>();
        let peak = output.append(&bytes);
        assert_eq!(output.samples, vec![0.0, 0.0, 0.25]);
        assert!((peak - 0.25).abs() < f32::EPSILON);
    }
}
