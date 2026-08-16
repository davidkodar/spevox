mod application;
mod controller;

use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QQmlApplicationEngine, QQmlEngine, QUrl};

fn main() {
    let mut application = application::new_application();
    if application
        .as_ref()
        .is_none_or(|application| !application::is_primary_instance(application))
    {
        eprintln!("FluidVoice is already running; requested its settings window.");
        return;
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
        let qml_engine: std::pin::Pin<&mut QQmlEngine> = engine.upcast_pin();
        qml_engine.on_quit(|_| {}).release();
    }

    if let Some(application) = application.as_mut() {
        application::exec_application(application);
    }
}
