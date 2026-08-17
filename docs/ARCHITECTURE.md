# Architecture

## Product boundary

The Linux client is a native implementation for KDE Plasma on Wayland. The macOS FluidVoice source is a behavioral and algorithmic reference, not a portable application shell.

Optional post-transcription enhancement sits behind a provider-neutral boundary.
Cloud providers receive transcript text only after explicit opt-in; loopback
Ollama, LM Studio, and compatible servers support fully local cleanup. The
default local-only lock prevents accidental cloud selection. Fluid-1 remains a
separate closed component and is not reproduced here.

## Experience parity

Qt Quick/QML will reproduce the macOS client's visual language rather than defaulting to a generic utility UI. Shared design tokens will cover color, type scale, spacing, radii, shadows, translucency, animation curves, and status colors. Reference views will be compared at fixed sizes during UI review.

Platform metaphors are translated rather than copied blindly: the notch overlay becomes a non-focusable Plasma overlay, the macOS menu-bar experience becomes a StatusNotifierItem tray experience, and permission prompts follow KDE/portal conventions. Visual parity must never require bypassing Wayland security or breaking Plasma window behavior.

## Implemented modules

```text
crates/fluidvoice-app/      command-line diagnostics and integration checks
crates/fluidvoice-audio/    PipeWire devices, capture, conversion, buffering
crates/fluidvoice-transcription/ whisper.cpp and local-server ASR adapters
crates/fluidvoice-portal/   XDG Global Shortcuts and permission adapters
crates/fluidvoice-delivery/ clipboard validation and recovery
crates/fluidvoice-ui/       CXX-Qt bridge, Qt Quick tray, overlay, settings
```

Within `fluidvoice-ui`, the CXX-Qt controller owns Qt-facing state and workflow
composition. Its larger implementation is organized across included Rust
source sections for settings and profiles, private storage, dictionary
processing, history, meeting transcription, speech-model catalogs, and speech
runtime helpers. These sections improve navigation but intentionally share the
controller module namespace; update checks and the bounded Whisper context
cache are independent Rust modules.

The UI controller also coordinates provider streaming, application/workflow
prompt profiles, selected-text rewriting, allowlisted Command Mode actions,
audio-file decoding, update checks, and release-facing status. These remain
separated from microphone capture and embedded whisper.cpp inference by typed
Rust boundaries even though the desktop bridge composes them in one process.

## Runtime state

```text
Idle → Recording → Transcribing → Delivering → Complete → Idle
  ↘      ↘              ↘             ↘
   Error/Cancelled/RecoverableClipboard
```

The CXX-Qt controller currently owns desktop workflow state. Feature-specific
busy properties keep ASR, assistant, update, and export work independent so an
unrelated background operation cannot disable dictation. Qt types remain at the
CXX-Qt/QML boundary and do not enter the audio, transcription, delivery, or
portal crates.

## First two technical gates

1. Demonstrate portal shortcut, PipeWire capture, whisper.cpp transcription, and clipboard output fully offline. KDE Global Shortcuts produced reliable press/release pairs for `Ctrl+Alt+D`, including release-driven PipeWire shutdown. PipeWire capture has source discovery, bounded native F32 capture, and a mono 16 kHz ASR conversion boundary. CPU-only `whisper.cpp` inference transcribed the official test sample successfully. Clipboard output was read back by both the application and KDE Klipper. A non-empty physical shortcut-to-clipboard run remains pending because the connected Scarlett input supplied silence. Evidence is recorded under `docs/validation/`.
2. Measure AT-SPI and consented Wayland input behavior across KDE, terminal, browser, Electron, office, GTK, and XWayland applications.

The first Cargo-native Qt Quick shell is now implemented. Runtime integration proceeds without weakening either technical gate.
