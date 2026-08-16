use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(
        QmlModule::new("io.github.davidkodar.FluidVoiceLinux").qml_file("qml/Main.qml"),
    )
    .qrc_resources([
        "assets/fluidvoice-app.png",
        "assets/fluidvoice-tray.png",
        "assets/trash.svg",
    ])
    .qt_module("Network")
    .qt_module("QuickControls2")
    .qt_module("Widgets")
    .files(["src/application.rs", "src/controller.rs"])
    .cpp_file("src/application.cpp")
    .build();
}
