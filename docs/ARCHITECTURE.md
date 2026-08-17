# Architecture

## Product boundary

Spevox is a native implementation for KDE Plasma on Wayland. The macOS FluidVoice source is a behavioral and algorithmic reference, not a portable application shell.

Optional post-transcription enhancement sits behind a provider-neutral boundary.
Cloud providers receive transcript text only after explicit opt-in; loopback
Ollama, LM Studio, and compatible servers support fully local cleanup. The
default local-only lock prevents accidental cloud selection. Fluid-1 remains a
separate closed component and is not reproduced here.

## Implemented modules

```text
crates/spevox-cli/      command-line diagnostics and integration checks
crates/spevox-audio/    PipeWire devices, capture, conversion, buffering
crates/spevox-transcription/ whisper.cpp and local-server ASR adapters
crates/spevox-portal/   XDG Global Shortcuts and permission adapters
crates/spevox-delivery/ clipboard validation and recovery
crates/spevox-ui/       CXX-Qt bridge, Qt Quick tray, overlay, settings
```

Within `spevox-ui`, the CXX-Qt controller owns Qt-facing state and workflow
composition. Settings and profiles, private storage, dictionary processing,
history, meeting transcription, speech-model catalogs, speech-runtime helpers,
update checks, and the bounded Whisper context cache are real Rust modules with
explicit imports and parent-visible APIs rather than shared `include!`
namespaces.

The UI controller also coordinates provider streaming, application/workflow
prompt profiles, selected-text rewriting, allowlisted Command Mode actions,
audio-file decoding, update checks, and release-facing status. These remain
separated from microphone capture and embedded whisper.cpp inference by typed
Rust boundaries even though the desktop bridge composes them in one process.

## Session-bus profile boundary

Automatic profile selection is an opt-in KDE integration. Its D-Bus endpoint
is owned on the current user's session bus and accepts only bounded application
class and window-title strings; it does not accept text, commands, paths, or
credentials. The session bus is the sender trust boundary: another process
running as the same desktop user can report an identity, so profile selection
must never grant authority or trigger actions. It only chooses a cleanup prompt.
The packaged KWin script remains disabled until the user enables the feature.

The Flatpak manifest remains a developer preview. Direct KWin scripting and
host-native runtime installation are not claimed as sandbox-supported until a
portal-backed implementation and package-specific validation exist.

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
