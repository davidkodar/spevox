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
}

impl Default for FluidVoiceControllerRust {
    fn default() -> Self {
        Self {
            status_text: QString::from("Ready"),
            microphone_name: QString::from("PipeWire default input"),
            model_name: QString::from("Whisper Tiny · Multilingual"),
            recording: false,
            overlay_visible: false,
            audio_level: 0.0,
            stop_token: None,
        }
    }
}

impl ffi::FluidVoiceController {
    pub fn toggle_recording(mut self: Pin<&mut Self>) {
        if *self.as_ref().recording() {
            if let Some(token) = self.as_ref().rust().stop_token.as_ref() {
                token.stop();
            }
            self.set_status_text(QString::from("Finishing…"));
            return;
        }

        let stop_token = CaptureStopToken::new();
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
                None,
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
                            controller.as_mut().set_audio_level(level.clamp(0.0, 1.0));
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
                                .set_audio_level(audio.peak().clamp(0.0, 1.0));
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
