#![allow(clippy::unnecessary_box_returns)]

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
        type FluidVoiceController = super::FluidVoiceControllerRust;

        #[qinvokable]
        #[cxx_name = "toggleRecordingPreview"]
        fn toggle_recording_preview(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "setOverlayPreview"]
        fn set_overlay_preview(self: Pin<&mut Self>, visible: bool);
    }
}

use std::pin::Pin;

use cxx_qt_lib::QString;

pub struct FluidVoiceControllerRust {
    status_text: QString,
    microphone_name: QString,
    model_name: QString,
    recording: bool,
    overlay_visible: bool,
}

impl Default for FluidVoiceControllerRust {
    fn default() -> Self {
        Self {
            status_text: QString::from("Ready"),
            microphone_name: QString::from("Scarlett Solo · Input 1 Mic"),
            model_name: QString::from("Whisper Tiny · Multilingual"),
            recording: false,
            overlay_visible: false,
        }
    }
}

impl ffi::FluidVoiceController {
    pub fn toggle_recording_preview(mut self: Pin<&mut Self>) {
        let recording = !*self.as_ref().recording();
        self.as_mut().set_recording(recording);
        self.as_mut().set_status_text(QString::from(if recording {
            "Listening…"
        } else {
            "Ready"
        }));
        self.set_overlay_visible(recording);
    }

    pub fn set_overlay_preview(mut self: Pin<&mut Self>, visible: bool) {
        self.as_mut().set_overlay_visible(visible);
        if !visible && *self.as_ref().recording() {
            self.as_mut().set_recording(false);
            self.set_status_text(QString::from("Ready"));
        }
    }
}
