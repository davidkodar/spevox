#![allow(clippy::unnecessary_box_returns)]
#![allow(clippy::float_cmp)] // Generated Q_PROPERTY setter compares the value.

#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, status_text, cxx_name = "statusText")]
        #[qproperty(QString, microphone_name, cxx_name = "microphoneName")]
        #[qproperty(QString, model_name, cxx_name = "modelName")]
        #[qproperty(bool, recording)]
        #[qproperty(bool, overlay_visible, cxx_name = "overlayVisible")]
        #[qproperty(f32, audio_level, cxx_name = "audioLevel")]
        #[qproperty(f32, input_db, cxx_name = "inputDb")]
        #[qproperty(i32, audio_updates, cxx_name = "audioUpdates")]
        #[qproperty(QStringList, input_sources, cxx_name = "inputSources")]
        #[qproperty(i32, selected_input, cxx_name = "selectedInput")]
        #[qproperty(f32, gain_db, cxx_name = "gainDb")]
        #[qproperty(bool, transcribing)]
        #[qproperty(QString, transcript_text, cxx_name = "transcriptText")]
        type FluidVoiceController = super::FluidVoiceControllerRust;

        #[qinvokable]
        #[cxx_name = "toggleRecording"]
        fn toggle_recording(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "initializeAudio"]
        fn initialize_audio(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "selectInput"]
        fn select_input(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "setOverlayPreview"]
        fn set_overlay_preview(self: Pin<&mut Self>, visible: bool);
    }

    impl cxx_qt::Threading for FluidVoiceController {}
}

use std::{
    path::PathBuf,
    pin::Pin,
    time::{Duration, Instant},
};

use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{QString, QStringList};
use fluidvoice_audio::{AudioDevice, CaptureStopToken, PipeWireCapture};
use fluidvoice_transcription::{TranscriptionConfig, WhisperTranscriber};

pub struct FluidVoiceControllerRust {
    status_text: QString,
    microphone_name: QString,
    model_name: QString,
    recording: bool,
    overlay_visible: bool,
    audio_level: f32,
    input_db: f32,
    audio_updates: i32,
    input_sources: QStringList,
    selected_input: i32,
    gain_db: f32,
    transcribing: bool,
    transcript_text: QString,
    stop_token: Option<CaptureStopToken>,
    capture_target: Option<String>,
    devices: Vec<AudioDevice>,
}

impl Default for FluidVoiceControllerRust {
    fn default() -> Self {
        Self {
            status_text: QString::from("Ready"),
            microphone_name: QString::from("Detecting PipeWire inputs…"),
            model_name: QString::from("Whisper Tiny · Multilingual"),
            recording: false,
            overlay_visible: false,
            audio_level: 0.0,
            input_db: -60.0,
            audio_updates: 0,
            input_sources: QStringList::default(),
            selected_input: -1,
            gain_db: 0.0,
            transcribing: false,
            transcript_text: QString::default(),
            stop_token: None,
            capture_target: None,
            devices: Vec::new(),
        }
    }
}

impl ffi::FluidVoiceController {
    pub fn initialize_audio(self: Pin<&mut Self>) {
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let result = PipeWireCapture::devices();
            qt_thread
                .queue(move |mut controller| match result {
                    Ok(devices) => {
                        let preferred = devices.iter().find(|device| {
                            device.description.contains("Input 1")
                                || device.description.contains("Mic 1")
                        });
                        let selected = preferred.or_else(|| devices.first()).cloned();
                        if let Some(device) = selected {
                            let selected_index = devices
                                .iter()
                                .position(|candidate| candidate.node_name == device.node_name)
                                .and_then(|index| i32::try_from(index).ok())
                                .unwrap_or(0);
                            let names = devices
                                .iter()
                                .map(|device| QString::from(&device.description))
                                .collect::<QStringList>();
                            let rust = controller.as_mut().rust_mut().get_mut();
                            rust.capture_target = Some(device.node_name.clone());
                            rust.devices = devices;
                            controller.as_mut().set_input_sources(names);
                            controller.as_mut().set_selected_input(selected_index);
                            controller.set_microphone_name(QString::from(&device.description));
                        } else {
                            controller
                                .as_mut()
                                .set_microphone_name(QString::from("No PipeWire input found"));
                            controller.set_status_text(QString::from("No microphone available"));
                        }
                    }
                    Err(error) => {
                        controller
                            .as_mut()
                            .set_microphone_name(QString::from("PipeWire unavailable"));
                        controller.set_status_text(QString::from(&format!(
                            "Input detection failed: {error}"
                        )));
                    }
                })
                .ok();
        });
    }

    pub fn select_input(mut self: Pin<&mut Self>, index: i32) {
        if *self.as_ref().recording() {
            return;
        }
        if *self.as_ref().transcribing() {
            return;
        }
        let Ok(index_usize) = usize::try_from(index) else {
            return;
        };
        let Some(device) = self.as_ref().rust().devices.get(index_usize).cloned() else {
            return;
        };
        self.as_mut().rust_mut().get_mut().capture_target = Some(device.node_name);
        self.as_mut().set_selected_input(index);
        self.as_mut()
            .set_microphone_name(QString::from(&device.description));
        self.set_status_text(QString::from("Input selected · ready to test"));
    }

    #[allow(clippy::too_many_lines)]
    pub fn toggle_recording(mut self: Pin<&mut Self>) {
        if *self.as_ref().recording() {
            if let Some(token) = self.as_ref().rust().stop_token.as_ref() {
                token.stop();
            }
            self.set_status_text(QString::from("Finishing…"));
            return;
        }

        let stop_token = CaptureStopToken::new();
        let capture_target = self.as_ref().rust().capture_target.clone();
        let gain = 10.0_f32.powf(*self.as_ref().gain_db() / 20.0);
        self.as_mut().rust_mut().get_mut().stop_token = Some(stop_token.clone());
        self.as_mut().set_audio_level(0.0);
        self.as_mut().set_input_db(-60.0);
        self.as_mut().set_audio_updates(0);
        self.as_mut().set_transcript_text(QString::default());
        self.as_mut().set_recording(true);
        self.as_mut().set_status_text(QString::from("Listening…"));
        self.as_mut().set_overlay_visible(true);

        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let level_thread = qt_thread.clone();
            let mut last_level_report: Option<Instant> = None;
            let result = PipeWireCapture::capture_with_levels(
                Duration::from_mins(2),
                capture_target.as_deref(),
                &stop_token,
                move |level| {
                    if last_level_report
                        .is_some_and(|reported| reported.elapsed() < Duration::from_millis(50))
                    {
                        return;
                    }
                    last_level_report = Some(Instant::now());
                    level_thread
                        .queue(move |mut controller| {
                            let adjusted = level * gain;
                            controller.as_mut().set_audio_level(meter_level(adjusted));
                            controller.as_mut().set_input_db(peak_db(adjusted));
                            let updates = controller.as_ref().audio_updates().saturating_add(1);
                            controller.set_audio_updates(updates);
                        })
                        .ok();
                },
            );

            let audio = match result {
                Ok(audio) => audio,
                Err(error) => {
                    qt_thread
                        .queue(move |mut controller| {
                            controller.as_mut().rust_mut().get_mut().stop_token = None;
                            controller.as_mut().set_recording(false);
                            controller.as_mut().set_overlay_visible(false);
                            controller.as_mut().set_audio_level(0.0);
                            controller.set_status_text(QString::from(&format!(
                                "Capture failed: {error}"
                            )));
                        })
                        .ok();
                    return;
                }
            };

            let peak = audio.peak();
            let duration = audio.duration();
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().rust_mut().get_mut().stop_token = None;
                    controller.as_mut().set_recording(false);
                    controller.as_mut().set_transcribing(true);
                    controller
                        .as_mut()
                        .set_audio_level(meter_level(peak * gain));
                    controller.as_mut().set_input_db(peak_db(peak * gain));
                    controller.set_status_text(QString::from("Transcribing locally…"));
                })
                .ok();

            let mono = audio.to_asr_mono();
            let normalization_gain = automatic_asr_gain(mono.peak());
            let asr_audio = mono.amplified(gain * normalization_gain);
            let asr_peak = asr_audio.peak();
            let transcription = find_whisper_model().and_then(|model| {
                WhisperTranscriber::load(&model, TranscriptionConfig::default())
                    .map_err(|error| error.to_string())?
                    .transcribe(&asr_audio)
                    .map_err(|error| error.to_string())
            });
            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().set_transcribing(false);
                    controller.as_mut().set_overlay_visible(false);
                    match transcription {
                        Ok(transcript) if !transcript.text.is_empty() => {
                            controller
                                .as_mut()
                                .set_transcript_text(QString::from(&transcript.text));
                            controller.set_status_text(QString::from(&format!(
                                "Transcribed {:.1}s locally · ASR peak {:.0}%",
                                duration.as_secs_f32(),
                                asr_peak * 100.0
                            )));
                        }
                        Ok(_) => {
                            controller.as_mut().set_transcript_text(QString::from(
                                "No speech was recognized. Try speaking closer to the microphone.",
                            ));
                            controller.set_status_text(QString::from("No speech recognized"));
                        }
                        Err(error) => {
                            controller
                                .as_mut()
                                .set_transcript_text(QString::from(&format!(
                                    "Transcription failed: {error}"
                                )));
                            controller.set_status_text(QString::from("Transcription failed"));
                        }
                    }
                })
                .ok();
        });
    }

    pub fn set_overlay_preview(mut self: Pin<&mut Self>, visible: bool) {
        self.as_mut().set_overlay_visible(visible);
        if !visible && *self.as_ref().recording() {
            if let Some(token) = self.as_ref().rust().stop_token.as_ref() {
                token.stop();
            }
            self.set_status_text(QString::from("Finishing…"));
        }
    }
}

fn find_whisper_model() -> Result<PathBuf, String> {
    let configured = std::env::var_os("FLUIDVOICE_WHISPER_MODEL").map(PathBuf::from);
    let development =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../work/models/ggml-tiny.bin");
    let path = configured.unwrap_or(development);
    if path.is_file() {
        Ok(path)
    } else {
        Err(format!(
            "Whisper model not found at {}. Set FLUIDVOICE_WHISPER_MODEL.",
            path.display()
        ))
    }
}

fn meter_level(peak: f32) -> f32 {
    if !peak.is_finite() || peak <= 0.0 {
        return 0.0;
    }
    ((20.0 * peak.log10() + 60.0) / 60.0).clamp(0.0, 1.0)
}

fn peak_db(peak: f32) -> f32 {
    if !peak.is_finite() || peak <= 0.0 {
        return -60.0;
    }
    (20.0 * peak.log10()).clamp(-60.0, 0.0)
}

fn automatic_asr_gain(peak: f32) -> f32 {
    if !peak.is_finite() || peak <= 0.000_5 {
        return 1.0;
    }
    (0.5 / peak).clamp(1.0, 32.0)
}

#[cfg(test)]
mod tests {
    use super::{automatic_asr_gain, meter_level, peak_db};

    #[test]
    fn maps_audio_peak_to_logarithmic_meter() {
        assert_eq!(meter_level(0.0), 0.0);
        assert!((meter_level(0.01) - 1.0 / 3.0).abs() < 0.001);
        assert_eq!(meter_level(1.0), 1.0);
        assert_eq!(meter_level(f32::NAN), 0.0);
    }

    #[test]
    fn reports_bounded_decibels() {
        assert_eq!(peak_db(0.0), -60.0);
        assert!((peak_db(0.01) + 40.0).abs() < 0.001);
        assert_eq!(peak_db(1.0), 0.0);
    }

    #[test]
    fn normalizes_quiet_asr_audio_conservatively() {
        assert_eq!(automatic_asr_gain(0.0), 1.0);
        assert_eq!(automatic_asr_gain(0.01), 32.0);
        assert!((automatic_asr_gain(0.1) - 5.0).abs() < f32::EPSILON);
        assert_eq!(automatic_asr_gain(0.8), 1.0);
    }
}
