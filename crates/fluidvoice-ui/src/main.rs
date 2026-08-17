mod ai;
mod application;
mod controller;
mod local_api;
mod parakeet;

use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QQmlApplicationEngine, QQmlEngine, QUrl};

fn main() -> std::process::ExitCode {
    let mut application = application::new_application();
    let Some(application_ref) = application.as_ref() else {
        eprintln!("FluidVoice could not initialize the Qt application.");
        return std::process::ExitCode::FAILURE;
    };
    if !application::is_primary_instance(application_ref) {
        eprintln!("FluidVoice is already running; requested its settings window.");
        return std::process::ExitCode::SUCCESS;
    }
    let mut engine = QQmlApplicationEngine::new();

    if let Some(mut engine) = engine.as_mut() {
        {
            let mut qml_engine: std::pin::Pin<&mut QQmlEngine> = engine.as_mut().upcast_pin();
            qml_engine
                .as_mut()
                .set_output_warnings_to_standard_error(true);
        }
        engine.as_mut().load(&QUrl::from(
            "qrc:/qt/qml/io/github/davidkodar/FluidVoiceLinux/qml/Main.qml",
        ));
    }

    if let Some(mut application) = application.as_mut() {
        application::refresh_application_icon(application.as_mut());
        let code = application::exec_application(application);
        return u8::try_from(code).map_or(
            std::process::ExitCode::FAILURE,
            std::process::ExitCode::from,
        );
    }
    std::process::ExitCode::FAILURE
}
