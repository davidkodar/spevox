# Qt Quick desktop shell validation — 2026-08-15

## Environment

- Qt 6.11.1
- CXX-Qt 0.9.1
- KDE Plasma on Wayland
- QML module URI: `io.github.davidkodar.Spevox`

## Implemented shell

- Cargo-native Qt executable with no separate CMake application build
- Rust-backed `SpevoxController` exposed to QML through CXX-Qt
- Dark macOS-inspired settings surface with device/model summaries
- Compact frameless always-on-top recording overlay
- Recording/ready state preview driven through Rust invokables
- KDE system tray icon with open, preview, and quit actions
- Desktop entry updated to launch `spevox`

The controller currently supplies preview state and known development defaults. It does not yet own the asynchronous shortcut/capture/transcription workflow; that connection is intentionally the next integration slice.

## Checks

```bash
QMAKE=/usr/bin/qmake6 cargo build -p spevox-ui
qmllint -I target/cxxqt/qml_modules crates/spevox-ui/qml/Main.qml
QT_QPA_PLATFORM=offscreen timeout 5s target/debug/spevox
```

The application built against the installed Qt 6.11 stack, its packaged QML module remained running under the offscreen Wayland-independent platform plugin, and `qmllint` reported no errors.
