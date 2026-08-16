# FluidVoice Linux

Native, local-first voice dictation for KDE Plasma on Wayland.

> [!IMPORTANT]
> This is an unofficial Linux port inspired by
> [FluidVoice for macOS](https://github.com/altic-dev/FluidVoice). It is not
> sponsored or endorsed by Altic or the upstream maintainers. Development is
> now targeting 0.4.0; the project remains a private preview and is not yet
> packaged for general distribution.

FluidVoice Linux combines a Rust 2024 core, Qt Quick/CXX-Qt interface,
PipeWire capture, XDG desktop portals, and local whisper.cpp inference. The
normal dictation path does not send audio or transcripts to a cloud service.
Optional AI cleanup can use a local server or a provider the user explicitly
configures.

## Current features

- Hold-to-dictate from any application through a Plasma-managed global shortcut.
- Selectable PipeWire microphone with software gain, live dBFS meter, and input test.
- Local multilingual Whisper transcription across all 99 supported languages.
- Automatic language detection or a fixed language for more reliable short dictation.
- Tiny, Base, Small, Medium, Large Turbo, and Large v3 model management.
- Vulkan acceleration on compatible NVIDIA, AMD, and Intel GPUs with CPU fallback.
- Configurable live transcript overlay with animated typewriter-style updates,
  three sizes, placement, opacity, text visibility, and recovery actions.
- Wayland-aware direct paste where available, with verified clipboard recovery.
- Native Plasma system tray, single-instance activation, and correct desktop identity.
- Persistent custom spellings and optional spoken formatting commands.
- Local MP3, FLAC, Ogg/Opus, M4A/AAC, WebM, MP4, and WAV transcription through
  FFmpeg, plus transcript history and usage statistics.
- KDE system theme/accent defaults plus explicit FluidVoice dark, light, and accent options.
- Optional post-transcription cleanup through standard cloud providers, Ollama,
  LM Studio, or a custom OpenAI-compatible endpoint, with local model discovery
  and raw-text fallback.
- Persistent application/workflow profiles with profile-specific cleanup prompts.
- Upstream-style local statistics with configurable typing speed, weekend-aware
  streaks, 7/30-day activity, milestones, insights, records, and AI edit impact.
- Optional local microphone-audio history with a storage budget, automatic
  pruning, playback, individual deletion, and ZIP export.
- Optional authenticated loopback API for local automation and dictation control.
- Upstream-inspired settings navigation, onboarding, changelog, and feedback pages.

See [CHANGELOG.md](CHANGELOG.md) for release-by-release details.

## Quick start

### 1. Install build dependencies

The application currently requires:

- Rust 1.85 or newer with Cargo and rustfmt.
- Qt 6 Core, GUI, QML, Quick, Quick Controls 2, Network, Widgets, and `qmake6`.
- PipeWire development libraries.
- Vulkan loader, shader compiler, and development headers.
- `secret-tool` (from libsecret) when storing API keys for cloud AI providers.
- FFmpeg for broad audio-file transcription (16-bit PCM WAV retains a built-in fallback).

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
./packaging/install.sh
fluidvoice-ui
```

For a development run without installation:

```bash
QMAKE=/usr/bin/qmake6 cargo run -p fluidvoice-ui
```

Release tags publish a checksummed binary archive, an Arch `PKGBUILD`, stable
or beta update metadata, and Sigstore keyless signatures/certificates. Verify a
download with `cosign verify-blob --certificate-identity-regexp
'github.com/davidkodar/fluidvoice-linux' --certificate-oidc-issuer
'https://token.actions.githubusercontent.com' --certificate FILE.pem
--signature FILE.sig FILE`. The Flatpak manifest lives under
`packaging/flatpak`; after installing its KDE SDK/runtime, maintainers can build
an offline-vendored bundle with `./packaging/package-flatpak.sh`.

### 3. Configure dictation

1. Open **Voice Engine** and select the correct microphone.
2. Press **Test input** and confirm the meter follows your voice.
3. Download and activate a Whisper model. Base is a practical starting point
   for English; other languages may benefit from a larger model. Accuracy,
   memory use, and latency vary by language and hardware.
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

| Capability | Linux 0.4.0 development | Current macOS FluidVoice | Notes |
| --- | --- | --- | --- |
| Global hold-to-dictate | Available | Available | Linux uses the XDG Global Shortcuts portal. |
| Direct typing into applications | Partial | Available | Linux uses Wayland portal/clipboard delivery; application support can vary. |
| Live transcript overlay | Available | Available | Linux provides compact/standard/expanded layouts, placement, opacity, text visibility, streamed AI retry, copy, raw recovery, and undo; previews use periodic local Whisper rather than the newest Parakeet streaming path. |
| Whisper models | Available | Available | Linux supports all six listed multilingual GGML sizes. |
| Parakeet, Nemotron, Cohere, Apple Speech | Missing | Available | Linux currently ships only the Whisper engine. |
| Vulkan GPU acceleration | Available | Not applicable | Linux supports cross-vendor Vulkan with CPU fallback. |
| Automatic/fixed language | Available | Available | All 99 Whisper languages are exposed. |
| Model download and deletion | Available | Available | Downloads are size-validated and remain `.part` until complete. |
| Custom dictionary | Available | Available | Linux supports persistent spoken-to-preferred words and phrases, legacy capitalization entries, CSV/TSV import, CSV export, and explicit merge/overwrite/replace conflict policies. Dictionary replacements run deterministically after transcription. |
| Spoken formatting | Available | Available | Linux handles newline, paragraph, comma, period, question mark, and exclamation mark. |
| Command Mode system actions | Partial | Available | Linux provides provider-backed KDE assistance plus confirmed allowlisted actions for System Settings, Konsole, Dolphin, and screen locking; arbitrary shell execution is deliberately prohibited. |
| Write/rewrite selected text | Available | Available | `Ctrl+Alt+W` opens Write Mode from any application. Linux captures and replaces selected text through the consented Wayland keyboard portal, with provider/model visibility plus retry, undo, preview, and clipboard recovery. The closed Fluid-1 model is not available. |
| AI Enhancement | Partial | Available | Linux supports configurable cloud and local providers, streamed overlay updates, application/workflow prompt profiles, and safe raw-text fallback. It cannot reproduce the closed Fluid-1 model. |
| Fluid Intelligence / Fluid-1 | Missing | Available | Fluid-1 weights, runtime, and training code are separately maintained and not available in the GPL repository. |
| File and meeting transcription | Available | Available | Linux accepts MP3, FLAC, Ogg/Opus, M4A/AAC, WebM, MP4, and varied WAV through FFmpeg, then runs bounded timestamped Whisper segments with progress/cancellation and TXT, Markdown, SRT, WebVTT, or JSON export. Speaker fields are diarization-ready but honestly remain unassigned until a real backend is added. |
| Transcript history | Partial | Available | Linux provides search, timestamps, raw/final visual diffs, change counts, copy/undo actions, optional retained audio, AI metadata, word counts, and JSON/CSV/ZIP export; app/window metadata and feedback reports remain missing. |
| Usage statistics | Available | Available | Linux provides today/all-time totals, estimated time saved, configurable typing speed, weekend-aware streaks, 7/30-day charts, milestones, insights, records, and AI enhancement impact/provider activity. |
| Per-application configuration | Available | Available | Linux provides persistent named prompt profiles and opt-in automatic matching through a packaged KWin script. Only application class and window title cross the local session bus; the bridge is disabled by default. |
| Audio recording history | Available | Available | Optional and off by default; Linux retains local mono WAV recordings within a configurable budget, supports playback and deletion, and exports audio plus metadata as ZIP. |
| Adaptive themes and accents | Available | Available | System/KDE defaults plus explicit FluidVoice themes and colors. |
| Tray/menu-bar integration | Available | Available | Plasma tray is the Linux equivalent of the macOS menu bar item. |
| Automatic updates / beta channel | Partial | Available | Tagged Linux releases publish separate stable/beta metadata plus checksummed, keyless-signed artifacts. The app checks releases but does not yet install updates unattended; feeds remain unavailable while the repository is private. |

The macOS feature set changes quickly. This matrix reflects the upstream source
and README reviewed during 0.4.0 development; it is not a promise of identical
platform behavior.

## Models, languages, and storage

One multilingual Whisper model works for every exposed language. Managed model
downloads come from the official
[`ggml-org/whisper.cpp`](https://github.com/ggml-org/whisper.cpp) artifacts.

User data follows XDG conventions:

- Models: `$XDG_DATA_HOME/fluidvoice/models` or `~/.local/share/fluidvoice/models`.
- Dictionary: `$XDG_DATA_HOME/fluidvoice/dictionary.txt`.
- History: `$XDG_DATA_HOME/fluidvoice/history.tsv` (legacy entries are migrated
  compatibly as new enriched entries are appended).
- Settings: `$XDG_CONFIG_HOME/fluidvoice/settings.conf` or `~/.config/fluidvoice/settings.conf`.
- Application profiles: `$XDG_DATA_HOME/fluidvoice/ai-profiles.json`.
- Command history: `$XDG_DATA_HOME/fluidvoice/command-history.tsv`.
- Local API token: `$XDG_CONFIG_HOME/fluidvoice/local-api.token` (mode `0600`).

Audio is processed locally and is not retained unless optional Audio History is
explicitly enabled. Retained recordings use
`$XDG_DATA_HOME/fluidvoice/audio-history`, are capped by the selected budget,
and are removed when History is cleared. History and dictionary data can be
cleared from the interface or removed from the paths above.

AI enhancement is disabled by default. Ollama and LM Studio can keep the
cleanup step local, and FluidVoice can query either server for installed models.
When Ollama is selected, the AI Enhancement page distinguishes a missing
installation from a stopped server or an empty model library. It links to the
official Linux installer, can start `ollama serve`, and runs validated
`ollama pull` model downloads with an in-app busy/result state. Operating-system
installation still requires the user's explicit action and is never performed
silently with administrator privileges.
Their built-in presets reject non-loopback endpoints to prevent an accidental
privacy downgrade. The local-provider privacy lock is enabled by default and
must be explicitly switched off before a cloud provider can be selected. When
a cloud provider is enabled, the cleanup prompt and
raw transcript—not microphone audio—are sent to that provider. API keys are
stored through the desktop Secret Service using `secret-tool`; they are never
written to `settings.conf`. If enhancement is unavailable or fails, FluidVoice
delivers the unenhanced transcript. Provider responses are time-bounded,
size-limited, and transient failures are retried before fallback.

Automatic application profiles are also disabled by default. The installer
ships `fluidvoiceprofiles`, a small KWin script, but it is enabled only when the
user turns on **Automatic KWin profile selection**. KWin then reports the active
application class and window title to FluidVoice's session-local D-Bus endpoint.
Profile rules use comma-separated, case-insensitive fragments; blank rules stay
manual-only. Disabling the option disables the KWin script again.

## Local automation API

The local API is disabled by default. Enable it under **Settings → Local API**,
choose a non-privileged port, and restart FluidVoice. It binds exclusively to
IPv4 loopback (`127.0.0.1`), rejects browser `Origin` requests, limits request
headers, applies short I/O timeouts, and never accepts filesystem paths or shell
commands. Copy or rotate its 256-bit bearer token from the same settings card.

```bash
TOKEN="$(cat "${XDG_CONFIG_HOME:-$HOME/.config}/fluidvoice/local-api.token")"
curl http://127.0.0.1:43128/v1/health
curl -H "Authorization: Bearer $TOKEN" http://127.0.0.1:43128/v1/status
curl -X POST -H "Authorization: Bearer $TOKEN" \
  http://127.0.0.1:43128/v1/dictation/toggle
```

Only health is unauthenticated. Status and dictation control require the token;
rotating it immediately revokes existing clients.

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
cargo run -p fluidvoice-app -- --diagnose-text-delivery
cargo run -p fluidvoice-app -- --diagnose-audio 3 [PIPEWIRE_NODE]
cargo run -p fluidvoice-app -- --diagnose-transcription /path/to/model.bin 5 [PIPEWIRE_NODE]
cargo run -p fluidvoice-app -- --diagnose-workflow /path/to/model.bin 5 [PIPEWIRE_NODE]
```

The application ID is `io.github.davidkodar.FluidVoiceLinux`. The matching
desktop entry, icon, AppStream metadata, and GPL license are installed by
`packaging/install.sh`. Use `packaging/install-dev.sh` when iterating on a debug
build, or `packaging/uninstall.sh` to remove application files without deleting
models, settings, history, or other user data.

Tagged releases are built by GitHub Actions as versioned `x86_64.tar.gz`
archives with SHA-256 checksum files. `packaging/package-tarball.sh` reproduces
that artifact locally. The in-app update check reads GitHub Releases and will
remain unavailable while this repository is private; releases are not installed
silently.

The current production speech backend remains embedded whisper.cpp. Parakeet,
Nemotron, Cohere, and Apple Speech depend on runtimes or platform services that
cannot presently be shipped as a supported native KDE/Linux backend; the engine
boundary remains isolated so a maintainable Linux runtime can be added later
without changing capture, history, or delivery.

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

The core KDE Wayland dictation workflow, broad audio decoding, rich local
history, streamed AI cleanup, explicit application profiles, and checksummed
release packaging are operational. Before a general public release, priorities
are guided local-model setup, accessibility review, signed distribution-native
packages, and wider testing across Plasma distributions and GPU vendors.

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
