# FluidVoice Linux

Native, local-first voice dictation for KDE Plasma on Wayland.

This private repository contains the Linux client project. It is currently in the foundation and technical-spike phase; it is not ready for use or redistribution.

The product should preserve the macOS application's visual identity and interaction feel as closely as practical: compact translucent overlays, typography hierarchy, spacing, status transitions, animation timing, settings organization, and adaptive themes. Linux-specific permission, tray, shortcut, and window behavior remains native to KDE Plasma.

## First milestone

Prove the complete local workflow:

```text
KDE global shortcut
    → PipeWire microphone capture
    → whisper.cpp transcription
    → clipboard recovery
```

Reliable automatic insertion into Wayland applications is a separate validation milestone. The implementation will use capability detection and will not require root access or weakened Wayland security by default.

## Initial technology choices

- Rust 2024 and Cargo for the application core and Linux integrations
- Qt Quick/QML for the Plasma interface, connected through CXX-Qt
- PipeWire for microphone capture
- XDG Global Shortcuts portal for hold/toggle activation
- whisper.cpp through Rust bindings, with a CPU baseline and optional acceleration
- AT-SPI and consented Wayland interfaces for text-delivery research

## Build the foundation

Requirements for the current foundation:

- Rust 1.85 or newer with Cargo, rustfmt, and Clippy

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p fluidvoice-app
```

## Status

The current workspace establishes the platform-independent dictation state machine and a thin application binary. Qt/QML, audio, shortcuts, transcription, and text delivery have not yet been implemented.

## Upstream relationship and licensing

The project is informed by the GPLv3-licensed [FluidVoice](https://github.com/altic-dev/FluidVoice) macOS application. Before the first redistribution, this repository will include the complete license and attribution inventory for all copied or adapted upstream material, dependencies, and speech models.

Fluid Intelligence is a separate private component and is not part of this project.
