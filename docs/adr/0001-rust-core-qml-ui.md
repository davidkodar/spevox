# ADR 0001: Rust core with a Qt Quick/QML interface

- Status: accepted
- Date: 2026-08-15

## Context

FluidVoice Linux needs reliable asynchronous state handling, real-time audio integration, D-Bus portals, accessibility integration, local inference, and a Plasma-appropriate interface.

C++ provides the most direct Qt and KDE integration. Rust provides stronger safety guarantees and more explicit concurrency and ownership boundaries for the application's higher-risk subsystems. Current Rust libraries cover PipeWire, XDG portals/D-Bus, AT-SPI, and whisper.cpp. CXX-Qt supports exposing Rust objects to Qt/QML, although its API is less mature than Qt's C++ API.

## Decision

Implement the application core and Linux integrations in Rust. Build the Plasma interface with Qt Quick/QML and bridge it to Rust using CXX-Qt. Keep the bridge narrow and prevent Qt types from entering the domain model.

The QML layer will intentionally reproduce FluidVoice's macOS visual identity and interaction timing through explicit design tokens and visual baselines. KDE-native behavior takes precedence for permissions, tray integration, focus, accessibility, and Wayland security boundaries.

The first vertical slice will remain headless so that shortcut, capture, transcription, and clipboard behavior can be proven before the UI dependency is introduced.

## Consequences

- Memory and thread-safety risks are reduced in the audio, state-machine, and delivery layers.
- Cargo becomes the primary build and test interface.
- Qt/QML integration requires generated C++ glue and a compatible native Qt toolchain.
- CXX-Qt upgrades must be deliberate because it is still evolving.
- If the bridge becomes a release blocker, a very small handwritten C++ Qt shell may replace it without rewriting the Rust core.
