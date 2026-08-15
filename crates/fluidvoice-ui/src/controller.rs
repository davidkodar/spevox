#![allow(clippy::unnecessary_box_returns)]
#![allow(clippy::float_cmp)] // Generated Q_PROPERTY setter compares the value.

#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, status_text)]
        #[qproperty(QString, microphone_name)]
        #[qproperty(QString, model_name)]
        #[qproperty(bool, recording)]
        #[qproperty(bool, overlay_visible)]
        #[qproperty(f32, audio_level)]
        type FluidVoiceController = super::FluidVoiceControllerRust;

        #[qinvokable]
        #[cxx_name = "toggleRecording"]
        fn toggle_recording(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "initializeAudio"]
        fn initialize_audio(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "setOverlayPreview"]
        fn set_overlay_preview(self: Pin<&mut Self>, visible: bool);
    }

    impl cxx_qt::Threading for FluidVoiceController {}
}

use std::{
    pin::Pin,
    time::{Duration, Instant},
};

use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use fluidvoice_audio::{CaptureStopToken, PipeWireCapture};

pub struct FluidVoiceControllerRust {
    status_text: QString,
    microphone_name: QString,
    model_name: QString,
    recording: bool,
    overlay_visible: bool,
    audio_level: f32,
    stop_token: Option<CaptureStopToken>,
    capture_target: Option<String>,
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
            stop_token: None,
            capture_target: None,
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
                        let selected = preferred.or_else(|| devices.first());
                        if let Some(device) = selected {
                            controller.as_mut().rust_mut().get_mut().capture_target =
                                Some(device.node_name.clone());
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
        self.as_mut().rust_mut().get_mut().stop_token = Some(stop_token.clone());
        self.as_mut().set_audio_level(0.0);
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
                            controller.as_mut().set_audio_level(meter_level(level));
                        })
                        .ok();
                },
            );

            qt_thread
                .queue(move |mut controller| {
                    controller.as_mut().rust_mut().get_mut().stop_token = None;
                    controller.as_mut().set_recording(false);
                    controller.as_mut().set_overlay_visible(false);
                    match result {
                        Ok(audio) => {
                            controller
                                .as_mut()
                                .set_audio_level(meter_level(audio.peak()));
                            controller.set_status_text(QString::from(&format!(
                                "Captured {:.1}s · peak {:.0}%",
                                audio.duration().as_secs_f32(),
                                audio.peak() * 100.0
                            )));
                        }
                        Err(error) => {
                            controller.as_mut().set_audio_level(0.0);
                            controller.set_status_text(QString::from(&format!(
                                "Capture failed: {error}"
                            )));
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

fn meter_level(peak: f32) -> f32 {
    if !peak.is_finite() || peak <= 0.0 {
        return 0.0;
    }
    ((20.0 * peak.log10() + 60.0) / 60.0).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::meter_level;

    #[test]
    fn maps_audio_peak_to_logarithmic_meter() {
        assert_eq!(meter_level(0.0), 0.0);
        assert!((meter_level(0.01) - 1.0 / 3.0).abs() < 0.001);
        assert_eq!(meter_level(1.0), 1.0);
        assert_eq!(meter_level(f32::NAN), 0.0);
    }
}
