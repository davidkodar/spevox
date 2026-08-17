use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    // CXX-Qt embeds these files in the executable's Qt resource collection.
    // Track them explicitly so replacing branding assets cannot leave a stale
    // icon inside an otherwise up-to-date binary.
    for resource in [
        "assets/spevox-app.png",
        "assets/spevox-tray.png",
        "assets/trash.svg",
        "qml/Main.qml",
    ] {
        println!("cargo:rerun-if-changed={resource}");
    }

    let builder = CxxQtBuilder::new_qml_module(
        QmlModule::new("io.github.davidkodar.Spevox").qml_file("qml/Main.qml"),
    )
    .qrc_resources([
        "assets/spevox-app.png",
        "assets/spevox-tray.png",
        "assets/trash.svg",
    ])
    .qt_module("Network")
    .qt_module("QuickControls2")
    .qt_module("Widgets")
    .files(["src/application.rs", "src/controller.rs"])
    .cpp_file("src/application.cpp");

    // GCC 16 diagnoses a Qt 6 QChar forward-declaration pattern in generated
    // CXX-Qt code. Qt is valid here and the warning originates entirely in
    // system/generated headers, so suppress only this named diagnostic while
    // retaining every other C++ warning.
    unsafe {
        builder
            .cc_builder(|compiler| {
                compiler.flag_if_supported("-Wno-sfinae-incomplete");
            })
            .build();
    }
}
