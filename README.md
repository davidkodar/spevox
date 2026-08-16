# FluidVoice Linux

Native, local-first voice dictation for KDE Plasma on Wayland.

This private repository contains an **unofficial Linux port** of FluidVoice. It
is not sponsored or endorsed by Altic or the upstream FluidVoice maintainers.

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

## Build and run

Requirements for the current foundation:

- Rust 1.85 or newer with Cargo, rustfmt, and Clippy
- Qt 6 with Core, GUI, QML, Quick Controls 2, Network, and `qmake6`

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p fluidvoice-app
```

Launch the first Qt Quick desktop shell with:

```bash
QMAKE=/usr/bin/qmake6 cargo run -p fluidvoice-ui
```

The shell includes the settings surface, KDE tray menu, macOS-inspired compact recording overlay, and a Rust-backed CXX-Qt state controller. The preview button exercises QML state transitions; connecting those states to the already validated dictation pipeline is the next integration slice.

### Languages and local models

The Voice Engine settings expose Automatic detection plus all 99 languages
supported by multilingual Whisper. A language does not require its own model:
one downloaded multilingual model can be used for every listed language.

FluidVoice Linux can download, validate, activate, cancel, and delete the
official Tiny, Base, Small, Medium, Large Turbo, and Large v3 GGML artifacts
published by [`ggml-org/whisper.cpp`](https://github.com/ggml-org/whisper.cpp).
Managed downloads are stored under `$XDG_DATA_HOME/fluidvoice/models`, or
`~/.local/share/fluidvoice/models` when `XDG_DATA_HOME` is unset. Files remain
as `.part` until the exact expected artifact size has been received.

On KDE Plasma Wayland, the current shortcut diagnostic creates a portal-managed binding and prints separate press/release events:

```bash
cargo run -p fluidvoice-app -- --diagnose-shortcut
```

Plasma may display a shortcut configuration dialog the first time. The suggested trigger is `Ctrl+Alt+D`; the desktop remains authoritative and may let the user choose another binding.

The audio diagnostic lists available PipeWire microphone sources, captures three seconds from the default source, and reports both the native signal and normalized mono 16 kHz ASR boundary:

```bash
cargo run -p fluidvoice-app -- --diagnose-audio 3
```

To select a specific microphone, append its PipeWire node name as the final argument.

Local transcription uses a `whisper.cpp` GGML model supplied by path. After downloading a model from the official `whisper.cpp` model repository, capture and transcribe five seconds with:

```bash
cargo run -p fluidvoice-app -- --diagnose-transcription /path/to/ggml-tiny.bin 5 [PIPEWIRE_NODE]
```

The current baseline deliberately uses CPU inference. Model download/selection, hardware acceleration, and language controls will become application settings rather than being hard-coded into the engine.

The first complete recovery workflow waits for the KDE shortcut, records a bounded capture, transcribes locally, and copies verified non-empty text to the clipboard:

```bash
cargo run -p fluidvoice-app -- --diagnose-workflow /path/to/ggml-tiny.bin 5 [PIPEWIRE_NODE]
```

This diagnostic records while the shortcut is held and stops PipeWire capture on release, with a configurable safety ceiling. Captures shorter than 300 ms are treated as accidental taps and leave the clipboard unchanged. The process intentionally remains alive after copying so it can continue serving the Wayland clipboard selection until the text is pasted or replaced.

The application ID is `io.github.davidkodar.FluidVoiceLinux`. The development diagnostic registers this host identity with the portal before requesting a shortcut; release packaging will install the matching desktop entry from `data/`.

## Status

The current workspace establishes the platform-independent dictation state machine, KDE global-shortcut integration, PipeWire microphone capture, the mono 16 kHz ASR boundary, local `whisper.cpp` transcription, verified KDE Wayland clipboard recovery, and a launchable Qt Quick/CXX-Qt desktop shell. Connecting the GUI to the runtime pipeline and automatic text insertion remain to be implemented.

## Upstream relationship and licensing

This project is licensed under GPLv3 and is based on the GPLv3-licensed
[FluidVoice](https://github.com/altic-dev/FluidVoice) macOS application. It
reuses the upstream application and menu-bar icons under GPLv3. See
[`LICENSE`](LICENSE) and [`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md) for
the license and exact asset provenance.

Fluid Intelligence is a separate private component and is not part of this project.
