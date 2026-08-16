# FluidVoice Linux

Native, local-first voice dictation for KDE Plasma on Wayland.

> [!IMPORTANT]
> This is an unofficial Linux port inspired by
> [FluidVoice for macOS](https://github.com/altic-dev/FluidVoice). It is not
> sponsored or endorsed by Altic or the upstream maintainers. Version 0.2.0 is
> a private preview and is not yet packaged for general distribution.

FluidVoice Linux combines a Rust 2024 core, Qt Quick/CXX-Qt interface,
PipeWire capture, XDG desktop portals, and local whisper.cpp inference. The
normal dictation path does not send audio or transcripts to a cloud service.

## Current features

- Hold-to-dictate from any application through a Plasma-managed global shortcut.
- Selectable PipeWire microphone with software gain, live dBFS meter, and input test.
- Local multilingual Whisper transcription across all 99 supported languages.
- Automatic language detection or a fixed language for more reliable short dictation.
- Tiny, Base, Small, Medium, Large Turbo, and Large v3 model management.
- Vulkan acceleration on compatible NVIDIA, AMD, and Intel GPUs with CPU fallback.
- Live transcript overlay with animated typewriter-style updates.
- Wayland-aware direct paste where available, with verified clipboard recovery.
- Native Plasma system tray, single-instance activation, and correct desktop identity.
- Persistent custom spellings and optional spoken formatting commands.
- Local 16-bit PCM WAV transcription, transcript history, and basic usage statistics.
- KDE system theme/accent defaults plus explicit FluidVoice dark, light, and accent options.
- Upstream-inspired settings navigation, onboarding, changelog, and feedback pages.

See [CHANGELOG.md](CHANGELOG.md) for release-by-release details.

## Quick start

### 1. Install build dependencies

The application currently requires:

- Rust 1.85 or newer with Cargo and rustfmt.
- Qt 6 Core, GUI, QML, Quick, Quick Controls 2, Network, Widgets, and `qmake6`.
- PipeWire development libraries.
- Vulkan loader, shader compiler, and development headers.

On Arch Linux or CachyOS, the Vulkan headers are provided by:

```bash
sudo pacman -S --needed vulkan-headers
```

### 2. Build and install

Run the installer as your normal user. It builds with your Rust toolchain and
uses `sudo` only when copying files into `/usr/local`.

```bash
git clone https://github.com/davidkodar/fluidvoice-linux.git
cd fluidvoice-linux
./packaging/install-dev.sh
fluidvoice-ui
```

For a development run without installation:

```bash
QMAKE=/usr/bin/qmake6 cargo run -p fluidvoice-ui
```

### 3. Configure dictation

1. Open **Voice Engine** and select the correct microphone.
2. Press **Test input** and confirm the meter follows your voice.
3. Download and activate a Whisper model. Small is a useful starting point for
   Swedish; larger models improve accuracy at the cost of memory and latency.
4. Select a fixed language for reliable short dictation, or use Automatic for
   mixed-language speech.
5. Leave compute on **Automatic (Vulkan)** for GPU acceleration with safe CPU
   fallback.
6. Hold the configured shortcut (initially `Ctrl+Alt+D`), speak, and release it
   to transcribe and paste.

Plasma may ask you to approve or choose the shortcut the first time. Closing
the settings window leaves FluidVoice running in the system tray.

## Linux and macOS feature comparison

This table is intentionally conservative. “Partial” means the Linux feature is
usable but does not yet match the depth of the current macOS implementation.

| Capability | Linux 0.2.0 | Current macOS FluidVoice | Notes |
| --- | --- | --- | --- |
| Global hold-to-dictate | Available | Available | Linux uses the XDG Global Shortcuts portal. |
| Direct typing into applications | Partial | Available | Linux uses Wayland portal/clipboard delivery; application support can vary. |
| Live transcript overlay | Available | Available | Linux uses periodic local Whisper previews rather than the newest Parakeet streaming path. |
| Whisper models | Available | Available | Linux supports all six listed multilingual GGML sizes. |
| Parakeet, Nemotron, Cohere, Apple Speech | Missing | Available | Linux currently ships only the Whisper engine. |
| Vulkan GPU acceleration | Available | Not applicable | Linux supports cross-vendor Vulkan with CPU fallback. |
| Automatic/fixed language | Available | Available | All 99 Whisper languages are exposed. |
| Model download and deletion | Available | Available | Downloads are size-validated and remain `.part` until complete. |
| Custom dictionary | Partial | Available | Linux applies persistent preferred capitalization after transcription; deeper model prompting is planned. |
| Spoken formatting | Available | Available | Linux handles newline, paragraph, comma, period, question mark, and exclamation mark. |
| Command Mode system actions | Missing | Available | Linux does not yet launch apps or execute desktop workflows by voice. |
| Write/rewrite selected text | Missing | Available | Requires a richer Wayland text-selection and editing integration. |
| AI Enhancement / Fluid Intelligence | Missing | Available | Fluid Intelligence is separately maintained and closed source; no cloud provider integration is included. |
| File transcription | Partial | Available | Linux currently accepts 16-bit PCM WAV; broader formats and meeting workflows are planned. |
| Transcript history | Partial | Available | Linux stores text locally; search, metadata, audio retention, ZIP export, and report flows are missing. |
| Usage statistics | Partial | Available | Linux currently reports transcript and word totals, not daily charts or WPM analysis. |
| Per-application configuration | Missing | Available | No app-specific prompt or behavior profiles yet. |
| Audio recording history | Missing | Available | Linux does not retain microphone audio after transcription. |
| Adaptive themes and accents | Available | Available | System/KDE defaults plus explicit FluidVoice themes and colors. |
| Tray/menu-bar integration | Available | Available | Plasma tray is the Linux equivalent of the macOS menu bar item. |
| Automatic updates / beta channel | Missing | Available | Planned after public packaging and signed releases exist. |

The macOS feature set changes quickly. This matrix reflects the upstream source
and README reviewed for the 0.2.0 preview; it is not a promise of identical
platform behavior.

## Models, languages, and storage

One multilingual Whisper model works for every exposed language. Managed model
downloads come from the official
[`ggml-org/whisper.cpp`](https://github.com/ggml-org/whisper.cpp) artifacts.

User data follows XDG conventions:

- Models: `$XDG_DATA_HOME/fluidvoice/models` or `~/.local/share/fluidvoice/models`.
- Dictionary: `$XDG_DATA_HOME/fluidvoice/dictionary.txt`.
- History: `$XDG_DATA_HOME/fluidvoice/history.tsv`.
- Settings: `$XDG_CONFIG_HOME/fluidvoice/settings.conf` or `~/.config/fluidvoice/settings.conf`.

Audio is processed locally and is not retained by the normal dictation flow.
History and dictionary data can be cleared from the interface or removed from
the paths above.

## GPU acceleration

**Automatic (Vulkan)** is the default compute mode. whisper.cpp uses a compatible
GPU when one is available and falls back to CPU otherwise. **CPU** disables GPU
initialization explicitly.

Successful GPU initialization includes output similar to:

```text
ggml_vulkan: 0 = NVIDIA GeForce RTX 4080
whisper_backend_init_gpu: using Vulkan0 backend
```

If FluidVoice reports `no GPU found`, verify the system independently:

```bash
nvidia-smi              # NVIDIA only
vulkaninfo --summary
```

## Development and diagnostics

Run the test suite and formatting checks with:

```bash
cargo fmt --all --check
cargo test --workspace
```

Useful diagnostics include:

```bash
cargo run -p fluidvoice-app -- --diagnose-shortcut
cargo run -p fluidvoice-app -- --diagnose-audio 3 [PIPEWIRE_NODE]
cargo run -p fluidvoice-app -- --diagnose-transcription /path/to/model.bin 5 [PIPEWIRE_NODE]
cargo run -p fluidvoice-app -- --diagnose-workflow /path/to/model.bin 5 [PIPEWIRE_NODE]
```

The application ID is `io.github.davidkodar.FluidVoiceLinux`. The matching
desktop entry, icon, AppStream metadata, and GPL license are installed by the
development installer.

## Architecture

- Rust provides capture, transcription, persistence, portals, model downloads,
  delivery, and state management.
- Qt Quick/QML provides the Plasma interface and compact recording overlay.
- CXX-Qt exposes typed Rust controller state to QML.
- PipeWire captures audio and converts it to mono 16 kHz floating-point samples.
- whisper.cpp performs local inference through Vulkan or CPU backends.
- XDG desktop portals provide the global shortcut and consented Wayland input path.

More detail is available in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Project status and roadmap

The core KDE Wayland dictation workflow is operational. Before a general public
release, priorities include broader audio formats, richer history and stats,
robust packaging, accessibility review, automated release artifacts, and wider
testing across Plasma distributions and GPU vendors. AI Enhancement is outside
the current open-source Linux scope unless a compatible provider is implemented.

Please use [GitHub Issues](https://github.com/davidkodar/fluidvoice-linux/issues)
for reproducible bugs and feature proposals once the repository is public.

## Upstream relationship and licensing

This project is GPLv3 and is based on the GPLv3-licensed
[FluidVoice](https://github.com/altic-dev/FluidVoice) macOS application. It
reuses upstream application and menu-bar icons under GPLv3. See [LICENSE](LICENSE)
and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for license and asset
provenance details.

Fluid Intelligence is a separate, privately maintained component and is not
part of this repository. FluidVoice names and artwork remain associated with
their respective owners; this Linux port clearly identifies itself as unofficial.
