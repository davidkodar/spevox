<p align="center">
  <img src="data/branding/spevox-logo.png" alt="Spevox" width="360">
</p>

Native, local-first voice dictation for KDE Plasma on Wayland.

> [!IMPORTANT]
> Spevox is an independent Linux application inspired by
> [FluidVoice for macOS](https://github.com/altic-dev/FluidVoice). It is not
> sponsored or endorsed by Altic or the upstream maintainers. Spevox is
> authored by David Bolin and informed by the upstream GPLv3 source and user
> experience. It remains a prerelease preview.

Credit belongs to Altic and the upstream FluidVoice contributors for the
original macOS project and product concept. Spevox uses its own name and
original artwork; historical artwork provenance from earlier private preview
builds remains recorded in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). See
[CREDITS.md](CREDITS.md) for authorship and relationship details.

Spevox combines a Rust 2024 core, Qt Quick/CXX-Qt interface,
PipeWire capture, XDG desktop portals, and local whisper.cpp inference. The
normal dictation path does not send audio or transcripts to a cloud service.
Optional AI cleanup can use a local server or a provider the user explicitly
configures.

## Current features

- Hold-to-dictate from any application through a Plasma-managed global shortcut.
- Selectable PipeWire microphone with software gain, live dBFS meter, and input test.
- Local multilingual Whisper transcription across all 99 supported languages.
- Experimental loopback-only OpenAI-compatible speech-server backend for
  user-managed local ASR runtimes.
- Automatic language detection or a fixed language for more reliable short dictation.
- Tiny, Base, Small, Medium, Large Turbo, and Large v3 model management.
- Vulkan acceleration on compatible NVIDIA, AMD, and Intel GPUs with CPU fallback.
- Configurable live transcript overlay with animated typewriter-style updates,
  direct editor delivery on shortcut release, and an optional persistent result
  popup with an upstream-style recovery actions menu,
  three sizes, placement, opacity, text visibility, and recovery actions.
- Wayland-aware direct paste where available, with verified clipboard recovery.
- Native Plasma system tray, single-instance activation, and correct desktop identity.
- Persistent custom spellings and optional spoken formatting commands.
- Local MP3, FLAC, Ogg/Opus, M4A/AAC, WebM, MP4, and WAV transcription through
  FFmpeg, plus transcript history and usage statistics.
- KDE system theme/accent defaults plus explicit Spevox dark, light, and accent options.
- Optional post-transcription cleanup through standard cloud providers, Ollama,
  LM Studio, or a custom OpenAI-compatible endpoint, with local model discovery
  and raw-text fallback.
- Conservative language-aware cleanup, deterministic spoken formatting,
  model-quality guidance, and observable raw/final results with provider,
  latency, language, policy, and fallback metadata.
- Persistent application/workflow profiles with profile-specific cleanup prompts.
- Upstream-style local statistics with configurable typing speed, weekend-aware
  streaks, 7/30-day activity, milestones, insights, records, and AI edit impact.
- Optional local microphone-audio history with a storage budget, automatic
  pruning, playback, individual deletion, and ZIP export.
- Optional authenticated loopback API for local automation and dictation control.
- Upstream-inspired settings navigation plus a first-run wizard with privacy,
  microphone, model speed/accuracy/size, GPU, experimental-engine, and test-
  dictation guidance; it remains reopenable from Getting Started.

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
git clone https://github.com/davidkodar/spevox.git
cd spevox
./packaging/install.sh
spevox
```

For a development run without installation:

```bash
QMAKE=/usr/bin/qmake6 cargo run -p spevox-ui
```

Release tags publish a checksummed binary archive and an Arch `PKGBUILD`.
Automated release runs also publish Sigstore keyless signatures/certificates.
Verify a signed
download with `cosign verify-blob --certificate-identity-regexp
'github.com/davidkodar/spevox' --certificate-oidc-issuer
'https://token.actions.githubusercontent.com' --certificate FILE.pem
--signature FILE.sig FILE`. The manifest under `packaging/flatpak` is a
developer preview and is not published as a release artifact: host KWin
integration, keyring access, native-runtime setup, and desktop actions still
need portal-native implementations before the sandboxed package is supported.

Initial packages are distributed through GitHub Releases rather than the AUR.
The release page provides a prebuilt x86_64 archive for supported Arch-family
systems and a standalone `PKGBUILD` for users who prefer a clean local source
build. The archive installer verifies linked runtime libraries before copying
files and stops with a dependency error instead of installing a binary that
cannot launch.

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
the settings window leaves Spevox running in the system tray.

## Linux and macOS feature comparison

This table is intentionally conservative. “Partial” means the Linux feature is
usable but does not yet match the depth of the current macOS implementation.

| Capability | Spevox 0.6.0 | Current macOS FluidVoice | Notes |
| --- | --- | --- | --- |
| Global hold-to-dictate | Available | Available | Linux uses the XDG Global Shortcuts portal. |
| Direct typing into applications | Partial | Available | Linux uses Wayland portal/clipboard delivery; application support can vary. |
| Live transcript overlay | Available | Available | Linux provides compact/standard/expanded layouts, placement, opacity, text visibility, direct editor delivery with automatic dismissal, and an optional persistent result/recovery menu. Streaming-capable native engines use NeMo's realtime endpoint for true partial results; built-in Whisper and full-utterance-only Parakeet TDT v3 use bounded Whisper previews. |
| Whisper models | Available | Available | Linux supports all six listed multilingual GGML sizes. |
| Parakeet, Nemotron, Cohere, Apple Speech | Partial | Available | Linux provides one-click managed Parakeet TDT v3, Nemotron 3.5 multilingual, Nemotron Streaming English, and Parakeet CTC 1.1B engines through native NeMo-Speech.cpp, plus a custom loopback ASR bridge. CoreML-only Cohere/Parakeet Flash and Apple Speech are not Linux runtimes. |
| Vulkan GPU acceleration | Available | Not applicable | Linux supports cross-vendor Vulkan with CPU fallback. |
| Automatic/fixed language | Available | Available | All 99 Whisper languages are exposed. |
| Model download and deletion | Available | Available | Downloads are SHA-256 verified, remain `.part` until complete, and retain an integrity marker for subsequent loads. |
| Custom dictionary | Available | Available | Linux supports persistent spoken-to-preferred words and phrases, legacy capitalization entries, CSV/TSV import, CSV export, and explicit merge/overwrite/replace conflict policies. Dictionary replacements run deterministically after transcription. |
| Spoken formatting | Available | Available | Linux handles newline, paragraph, comma, period, question mark, and exclamation mark. |
| Command Mode system actions | Partial | Available | Linux provides provider-backed KDE assistance plus confirmed allowlisted actions for System Settings, Konsole, Dolphin, and screen locking; arbitrary shell execution is deliberately prohibited. |
| Write/rewrite selected text | Available | Available | `Ctrl+Alt+W` opens Write Mode from any application. Linux captures and replaces selected text through the consented Wayland keyboard portal, with provider/model visibility plus retry, undo, preview, and clipboard recovery. The closed Fluid-1 model is not available. |
| AI Enhancement | Partial | Available | Linux supports configurable cloud and local providers, streamed overlay updates, application/workflow prompt profiles, and safe raw-text fallback. It cannot reproduce the closed Fluid-1 model. |
| Fluid Intelligence / Fluid-1 | Missing | Available | Fluid-1 weights, runtime, and training code are separately maintained and not available in the GPL repository. |
| File and meeting transcription | Available | Available | Linux accepts MP3, FLAC, Ogg/Opus, M4A/AAC, WebM, MP4, and varied WAV through FFmpeg, then runs bounded timestamped Whisper segments with progress/cancellation and TXT, Markdown, SRT, WebVTT, or JSON export. Optional experimental Sortformer v2 diarization labels up to four speakers locally; detected speakers can be renamed consistently in the current result, History, and exports, and the last file can be retried after changing settings. |
| Transcript history | Partial | Available | Linux provides search, timestamps, raw/final visual diffs, change counts, copy/undo actions, optional retained audio, AI metadata, word counts, and JSON/CSV/ZIP export; app/window metadata and feedback reports remain missing. |
| Usage statistics | Available | Available | Linux provides today/all-time totals, estimated time saved, configurable typing speed, weekend-aware streaks, 7/30-day charts, milestones, insights, records, and AI enhancement impact/provider activity. |
| Per-application configuration | Available | Available | Linux provides persistent named prompt profiles and opt-in automatic matching through a packaged KWin script. Only application class and window title cross the local session bus; the reporting KWin script is disabled by default. |
| Audio recording history | Available | Available | Optional and off by default; Linux retains local mono WAV recordings within a configurable budget, supports playback and deletion, and exports audio plus metadata as ZIP. |
| Adaptive themes and accents | Available | Available | System/KDE defaults plus explicit Spevox themes and colors. |
| Tray/menu-bar integration | Available | Available | Plasma tray is the Linux equivalent of the macOS menu bar item. |
| Automatic updates / beta channel | Partial | Available | The app checks public GitHub releases but does not yet install updates unattended or provide a separate beta feed. Hosted release automation remains manually triggered while CI capacity is limited. |

The macOS feature set changes quickly. This matrix reflects the upstream source
and README reviewed during 0.5.0 development; it is not a promise of identical
platform behavior.

## Models, languages, and storage

One multilingual Whisper model works for every exposed language. Managed model
downloads come from the official
[`ggml-org/whisper.cpp`](https://github.com/ggml-org/whisper.cpp) artifacts.

Built-in Whisper is the supported default. **Local speech server
(experimental)** accepts an OpenAI-compatible `/v1/audio/transcriptions`
service at HTTP loopback only. Spevox encodes the captured mono signal as
PCM WAV and will reject remote hosts, HTTPS URLs, recordings longer than two
minutes, or responses above 1 MiB. Live preview is unavailable because the
external process receives audio only after recording stops. This keeps native
ASR runtimes independently updateable; [sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx)
is a promising Linux/Rust route with offline, streaming, Parakeet/NeMo-class,
and diarization support, but 0.5.0 does not bundle its rapidly evolving native
runtime or model catalog.

**Native NVIDIA speech engines (beta)** are managed by the application without
Python, PyTorch, or a CUDA-only dependency chain. The catalog contains Parakeet
TDT v3, Nemotron 3.5 Multilingual, Nemotron Streaming English, and Parakeet CTC
1.1B. One click builds or reuses a pinned NVIDIA NeMo-Speech.cpp runtime,
downloads the chosen official quantized model, and verifies its exact size and
SHA-256 digest before activation. Nemotron 3.5 receives model-native locale
codes such as `sv-SE`; the two English-only engines deliberately force English.
“NVIDIA” identifies the runtime and model publisher, not a hardware requirement:
GPU acceleration uses cross-vendor Vulkan on supported AMD, Intel, and NVIDIA
drivers, requires no CUDA installation, and retains a CPU fallback.
The helper listens only on `127.0.0.1`, starts on first use, and is supervised
by Spevox. If setup, startup, or transcription fails, the captured audio is
sent through the selected local Whisper model instead of being lost. Installing
the runtime from source requires Git, CMake 3.26+, Ninja, and a C++17 compiler.
Its pinned private SentencePiece dependency is built automatically. Automatic
compute tries Vulkan first and installs CPU instead when the Vulkan development
packages are incomplete; “Vulkan only” remains available for strict GPU use.
During capture, streaming-capable native engines receive every audio chunk
through the managed loopback WebSocket and publish model-native partial text in the overlay. Cold
starts queue audio without blocking PipeWire, chunks are sent in NeMo's native
160 ms cadence, and release cancels the preview before the single authoritative
final transcription. Parakeet TDT v3 is explicitly full-utterance-only in the
pinned runtime, so it retains the periodic Whisper preview instead of opening
an unsupported stream. Whisper fallback remains available if the final fails.
With the currently pinned runtime, native realtime is enabled on CPU only: its
Vulkan streaming path can crash on real Nemotron audio in upstream GGML code.
Vulkan remains enabled for fast full-utterance final inference, while the live
overlay safely uses Whisper. Any native-engine failure is reported explicitly
before Spevox runs the slower Whisper recovery path.

**Speaker diarization (experimental)** is an optional file-transcription mode,
not part of the normal dictation hot path. One-click setup reuses the pinned
NeMo-Speech.cpp runtime and downloads a revision-pinned, SHA-256-verified GGUF
conversion of NVIDIA's Sortformer 4-speaker v2 checkpoint. It runs locally on
CPU or cross-vendor Vulkan, assigns each timestamped Whisper segment to the
speaker with the greatest time overlap, and carries those labels into History
and every meeting export format. If the model, runtime, or inference fails,
Spevox retains the complete Whisper transcript without speaker labels.
The consumable GGUF is generated by the documented Spevox release process
rather than published by NVIDIA and is labelled accordingly in the app.
Sortformer supports at most four
speakers and was trained primarily on English; noisy, overlapping, non-English,
or very long conversations may be less accurate.

Maintainers can reproduce the downloadable Q8 artifact with
`packaging/build-sortformer-model.sh`. The script pins both the NVIDIA
checkpoint and NeMo-Speech.cpp converter revisions, uses a temporary CPU-only
Python environment, verifies the source and output SHA-256 values, and leaves
only `target/package/sortformer-v2-q8_0.gguf`. Python and PyTorch are build-time
conversion tools only; neither is installed or invoked by Spevox.

User data follows XDG conventions:

- Models: `$XDG_DATA_HOME/spevox/models` or `~/.local/share/spevox/models`.
- Dictionary: `$XDG_DATA_HOME/spevox/dictionary.txt`.
- History: `$XDG_DATA_HOME/spevox/history.tsv` (legacy entries are migrated
  compatibly as new enriched entries are appended).
- Settings: `$XDG_CONFIG_HOME/spevox/settings.conf` or `~/.config/spevox/settings.conf`.
- Application profiles: `$XDG_DATA_HOME/spevox/ai-profiles.json`.
- Command history: `$XDG_DATA_HOME/spevox/command-history.tsv`.
- Local API token: `$XDG_CONFIG_HOME/spevox/local-api.token` (mode `0600`).

When upgrading from a 0.5.x private preview, Spevox continues using the legacy
`fluidvoice` XDG directory if no corresponding `spevox` directory exists. This
keeps settings, downloaded models, history, dictionaries, retained audio, API
tokens, and provider secrets available without copying or deleting user data.
New installations use the `spevox` paths above.

Audio is processed locally and is not retained unless optional Audio History is
explicitly enabled. Retained recordings use
`$XDG_DATA_HOME/spevox/audio-history`, are capped by the selected budget,
and are removed when History is cleared. History and dictionary data can be
cleared from the interface or removed from the paths above.

AI enhancement is disabled by default. Ollama and LM Studio can keep the
cleanup step local, and Spevox can query either server for installed models.
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
written to `settings.conf`. If enhancement is unavailable or fails, Spevox
delivers the unenhanced transcript. Provider responses are time-bounded,
size-limited, and transient failures are retried before fallback.

Automatic application profiles are also disabled by default. The installer
ships `spevoxprofiles`, a small KWin script, but it is enabled only when the
user turns on **Automatic KWin profile selection**. KWin then reports the active
application class and window title to Spevox's session-local D-Bus endpoint.
Profile rules use comma-separated, case-insensitive fragments; blank rules stay
manual-only. Disabling the option disables the KWin script again.

## Local automation API

The local API is disabled by default. Enable it under **Settings → Local API**,
choose a non-privileged port, and restart Spevox. It binds exclusively to
IPv4 loopback (`127.0.0.1`), rejects browser `Origin` requests, limits request
headers, applies short I/O timeouts, and never accepts filesystem paths or shell
commands. Copy or rotate its 256-bit bearer token from the same settings card.

```bash
TOKEN="$(cat "${XDG_CONFIG_HOME:-$HOME/.config}/spevox/local-api.token")"
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

If Spevox reports `no GPU found`, verify the system independently:

```bash
nvidia-smi              # NVIDIA only
vulkaninfo --summary
```

## Development and diagnostics

See [CONTRIBUTING.md](CONTRIBUTING.md) for the branch model, change workflow,
release process, and security expectations.

Run the test suite and formatting checks with:

```bash
cargo fmt --all --check
cargo test --workspace
```

Useful diagnostics include:

```bash
cargo run -p spevox-cli -- --diagnose-shortcut
cargo run -p spevox-cli -- --diagnose-text-delivery
cargo run -p spevox-cli -- --diagnose-audio 3 [PIPEWIRE_NODE]
cargo run -p spevox-cli -- --diagnose-transcription /path/to/model.bin 5 [PIPEWIRE_NODE]
cargo run -p spevox-cli -- --diagnose-workflow /path/to/model.bin 5 [PIPEWIRE_NODE]
```

The Voice Engine page also reports the selected CPU/Vulkan policy and provides
a refreshable compute diagnostic. This describes the requested runtime policy;
the transcription log remains the authoritative record of the backend actually
initialized by the model runtime.

Before cutting a release candidate, run `packaging/release-check.sh` for the
automated formatting, QML, test, release-build, staged-install, and package
checks. The manual fresh-user, accessibility, and optional-runtime acceptance
steps are documented in `docs/RELEASE_CHECKLIST.md`.

The application ID is `io.github.davidkodar.Spevox`. The matching
desktop entry, icon, AppStream metadata, and GPL license are installed by
`packaging/install.sh`. Use `packaging/install-dev.sh` when iterating on a debug
build, or `packaging/uninstall.sh` to remove application files without deleting
models, settings, history, or other user data.

Tagged releases are built by GitHub Actions as versioned `x86_64.tar.gz`
archives with SHA-256 checksum files. `packaging/package-tarball.sh` reproduces
that artifact locally. The in-app update check reads public GitHub Releases;
releases are never installed silently.

Embedded whisper.cpp remains the conservative default and automatic fallback.
The four managed NVIDIA engines are optional native KDE/Linux backends. Apple
Speech is an Apple platform service, while the currently published Cohere and
Parakeet Flash integrations rely on CoreML/Apple Neural Engine; they are shown
as platform gaps rather than nonfunctional Linux download buttons. Parakeet v2
is likewise omitted because NVIDIA does not publish a ready, verifiable GGUF
artifact for the pinned native runtime.

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
release packaging are operational. The public preview remains prerelease
software while guided local-model setup, accessibility review, signed
distribution-native packages, and testing across more Plasma distributions and
GPU vendors continue.

### Completed in 0.6.0

- **Independent Spevox identity:** renamed the Linux application from its
  former development name, introduced an original icon and wordmark, and
  migrated desktop, package, executable, and application identifiers while
  retaining access to existing 0.5.x settings, models, and history.

- **Structural refactoring and hardening:** controller responsibilities now use
  explicit Rust modules, blocking startup and persistence work is bounded or
  moved off the Qt thread, duplicated policies have been consolidated, and the
  release gate covers security, packaging, QML, and regression checks.

- **Multilingual intelligent dictation cleanup:** expanded the compact AI
  prompt into a constrained, upstream-inspired cleanup pipeline covering
  punctuation and question detection, capitalization, minimal unambiguous
  grammatical repairs, filler and false-start removal, self-corrections,
  spoken formatting, and number normalization. The implementation includes
  deterministic formatting where an LLM is unnecessary, language-aware prompt
  guidance, model recommendations, and regression fixtures that compare raw
  dictation with expected output across English, Swedish, and additional
  languages. It targets reliable behavior with local and cloud providers; it
  does not claim compatibility with or reproduction of the privately
  maintained Fluid Intelligence model.

Please use [GitHub Issues](https://github.com/davidkodar/spevox/issues)
for reproducible bugs and focused feature proposals.

## Upstream relationship and licensing

Spevox is an independent GPLv3 Linux implementation inspired by the
GPLv3-licensed [FluidVoice](https://github.com/altic-dev/FluidVoice) macOS
project created by Altic and the upstream contributors. Spevox is authored
and maintained by David Bolin and uses original branding. See
[LICENSE](LICENSE), [CREDITS.md](CREDITS.md), and
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for authorship, license, and
exact asset-provenance details. The generated
[THIRD_PARTY_LICENSES.html](THIRD_PARTY_LICENSES.html) bundles the license texts
and crate attribution for the locked Rust dependency graph.

Fluid Intelligence is a separate, privately maintained component and is not
part of this repository. FluidVoice and its artwork remain associated with
their respective owners; no trademark license or upstream endorsement is
claimed.
