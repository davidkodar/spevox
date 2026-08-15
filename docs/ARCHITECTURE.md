# Architecture

## Product boundary

The Linux client is a native implementation for KDE Plasma on Wayland. The macOS FluidVoice source is a behavioral and algorithmic reference, not a portable application shell.

## Planned modules

```text
app/                 application lifecycle and composition
ui/                  tray, overlay, settings, onboarding
core/                dictation state machine and domain events
audio/               PipeWire devices, capture, conversion, buffering
asr/                 provider interface and whisper.cpp adapter
shortcuts/            XDG Global Shortcuts portal adapter
delivery/             AT-SPI, consented input, clipboard recovery
text/                 deterministic formatting and dictionaries
persistence/          XDG settings, history, and migrations
platform/kde/         versioned KDE-specific adapters
tests/                unit and desktop integration tests
```

## State machine

```text
Idle → Recording → Transcribing → Delivering → Complete → Idle
  ↘      ↘              ↘             ↘
   Error/Cancelled/RecoverableClipboard
```

No platform integration should call directly across subsystem boundaries. The dictation coordinator owns state transitions and consumes interface-level events.

## First two technical gates

1. Demonstrate portal shortcut, PipeWire capture, whisper.cpp transcription, and clipboard output fully offline.
2. Measure AT-SPI and consented Wayland input behavior across KDE, terminal, browser, Electron, office, GTK, and XWayland applications.

Full UI and feature-parity work begins only after these gates establish a reliable core workflow.

