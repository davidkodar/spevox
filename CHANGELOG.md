# Changelog

All notable changes to FluidVoice Linux are documented here. The project uses
[Semantic Versioning](https://semver.org/) while it remains in private preview.

## 0.4.0 - 2026-08-16

- Added true realtime native-engine previews through NeMo-Speech.cpp's
  loopback WebSocket protocol, including lossless incremental PCM16 capture,
  cold-start buffering, cumulative partials, final-event reconciliation,
  160 ms model-native framing, locale/gain handling, and final Whisper fallback.
- Prevented realtime preview from delaying final transcription by eliminating
  per-PipeWire-chunk read waits and cancelling the preview session on release
  instead of redundantly running both WebSocket and HTTP final inference.
- Made realtime preview capability model-specific: Nemotron and buffered CTC
  use native partials, while full-utterance-only Parakeet TDT v3 keeps the
  established Whisper preview and no longer opens a rejected realtime session.
- Clarified in the native-engine interface that Vulkan acceleration works
  across AMD, Intel, and NVIDIA GPUs without requiring CUDA or NVIDIA hardware.
- Added a verified native speech catalog with one-click installation for
  Parakeet TDT v3, Nemotron 3.5 Multilingual, Nemotron Streaming English, and
  Parakeet CTC 1.1B. All engines reuse the pinned NeMo-Speech.cpp runtime,
  preserve Vulkan/CPU fallback and Whisper recovery, and use model-appropriate
  language identifiers.
- Made the AI Enhancement master switch authoritative for result retries and
  hid AI-only popup actions while enhancement is disabled.
- Restored type-as-you-speak text for the Parakeet engine by retaining the
  embedded Whisper live preview until Parakeet supplies the final transcript.
- Fixed Parakeet setup on current Arch toolchains by managing its pinned
  SentencePiece dependency, making Automatic genuinely fall back from missing
  Vulkan development files to CPU, showing build progress, shortening setup
  errors, and correcting the speech-engine card layout.
- Added a managed Parakeet TDT v3 beta engine using pinned NeMo-Speech.cpp,
  verified GGUF downloads, CPU/Vulkan runtimes, supervised loopback-only
  startup, and automatic Whisper fallback.
- Documented a post-0.4 multilingual intelligent-cleanup milestone with
  constrained upstream-inspired prompts, deterministic formatting, and
  language-spanning regression fixtures, while explicitly excluding claims of
  Fluid Intelligence compatibility.
- Redesigned the dictation result popup with responsive sizing, balanced visual
  hierarchy, equal-width actions, a dedicated dismiss control, and reliable
  suppression of listening and AI result popups when the setting is disabled.
- Reflowed statistics preferences into a bounded two-row layout so controls and
  labels remain inside the card, and made its typing-speed summary reflect the
  configured WPM value.
- Began the 0.4.0 development cycle with optional local audio retention,
  configurable storage budgets, oldest-first pruning, playback, individual
  deletion, and ZIP export with history metadata.
- Added compact, standard, and expanded overlay layouts, requested screen
  placement, opacity and live-text controls, plus result copy, raw recovery,
  undo, streamed AI retry, dismissal, and hover-aware timeout actions.
- Added a dedicated Plasma portal shortcut for Write Mode (`Ctrl+Alt+W`), an
  upstream-inspired writing and rewrite workspace, provider/model visibility,
  instruction-based drafting, selected-text replacement, result preview, retry,
  undo, and copy recovery controls.
- Made Wayland text delivery recover from stale keyboard-portal sessions,
  report direct-paste versus clipboard-only outcomes, and added non-destructive
  in-app and CLI delivery diagnostics with actionable Plasma error details.
- Added opt-in automatic AI profile selection through a packaged GPLv3 KWin
  script and local session-bus bridge. Profiles match application-class or
  title fragments; the bridge reports no document, transcript, or keyboard
  content and remains disabled by default.
- Upgraded the custom dictionary from capitalization-only terms to persistent
  spoken-to-preferred phrase replacements, while retaining legacy entries.
  Added quoted CSV/TSV import, CSV export, add-or-update behavior, and explicit
  keep-existing, overwrite-matches, and replace-all conflict policies.
- Completed the upstream-style statistics surface with configurable typing WPM,
  optional weekend-neutral streaks, 7/30-day activity views, milestones,
  peak-time insights, personal records, 30-day source/activity summaries,
  measured AI edit deltas, and confirmation-gated statistics reset.
- Added meeting and long-recording transcription with bounded 30-second local
  Whisper passes, timestamped segments, visible progress, cancellation, and
  TXT, Markdown, SRT, WebVTT, and JSON exports. The segment schema carries an
  optional speaker field for future diarization and labels it unassigned today
  rather than inventing speaker identities.
- Added an optional local automation API bound exclusively to `127.0.0.1`,
  disabled by default and protected by a rotatable 256-bit bearer token stored
  with owner-only permissions. It exposes only health, status, and dictation
  toggle operations, rejects browser origins, caps requests, and applies short
  connection timeouts.
- Added an Arch Linux package recipe, an offline-vendored Flatpak build path,
  stable/beta update manifests, and keyless Sigstore signing of tagged release
  artifacts through GitHub Actions. Automatic installation remains deliberately
  separate from update discovery.
- Added an experimental local speech-server engine using the standard
  `/v1/audio/transcriptions` contract. It accepts only HTTP loopback endpoints,
  caps dictation and response sizes, sends bounded PCM WAV after recording,
  and leaves built-in Whisper as the supported default. This provides a safe
  integration boundary for user-managed Linux ASR runtimes without bundling
  upstream's Apple-only CoreML/FluidAudio implementations.
- Added AI enhancement analytics for usage, success/fallback rate, average
  latency, and provider/model activity.
- Added per-entry raw/final visual diffs, word and character deltas, and copy
  raw, copy final, copy both, and undo-to-clipboard actions.

## 0.3.0 - 2026-08-16

### Added

- Added optional post-transcription AI cleanup for OpenAI, Anthropic, xAI,
  Groq, Cerebras, Gemini, OpenRouter, Ollama, LM Studio, and custom
  OpenAI-compatible endpoints.
- Added secure API-key storage through the desktop Secret Service, editable
  cleanup prompts, provider verification, and raw-transcript fallback.
- Added Ollama and LM Studio model discovery, installed-model selection, a
  fully-local privacy indicator, and loopback-only enforcement for local presets.
- Added guided Ollama diagnostics, local server startup, official installation
  guidance, and in-app model downloads with visible busy and result states.
- Added a persistent local-provider privacy lock, enabled by default, plus
  bounded provider responses and retries for transient failures.
- Enriched history with raw and final text, source, AI provider/model/status,
  processing latency, backward-compatible parsing, and JSON/CSV export.
- Added streamed AI cleanup updates in the recording overlay for
  OpenAI-compatible and Anthropic event-stream responses.
- Added persistent named application/workflow profiles with explicit selection
  and profile-specific cleanup prompts on Plasma Wayland.
- Added selected-text rewriting through the consented Wayland keyboard portal,
  configured AI provider, verified clipboard delivery, and paste fallback.
- Added provider-backed Command Mode assistance, persistent command history,
  and confirmation-gated allowlisted Plasma actions without arbitrary shell execution.
- Added bounded local FFmpeg decoding for MP3, FLAC, Ogg/Opus, M4A/AAC, WebM,
  MP4, and varied WAV files, with a built-in PCM WAV fallback.
- Added searchable history cards with relative and full local timestamps and word counts.
- Expanded local statistics with today's activity, estimated time saved, streaks,
  average session size, and a seven-day activity chart.

## 0.2.0 - 2026-08-16

### Added

- Vulkan transcription acceleration for NVIDIA, AMD, and Intel GPUs with CPU fallback.
- Automatic, GPU-preferred, and CPU compute modes.
- Download management for Whisper Tiny, Base, Small, Medium, Large Turbo, and Large v3.
- Automatic detection and fixed selection for all 99 Whisper languages.
- Persistent custom dictionary and optional spoken formatting commands.
- Local 16-bit PCM WAV transcription.
- Local transcript history, usage totals, onboarding, and feedback pages.
- KDE system, FluidVoice Dark, and FluidVoice Light themes.
- KDE system accent plus FluidVoice cyan, green, and purple accents.
- Upstream-style grouped navigation and a live typewriter-style transcript overlay.

### Changed

- Reworked settings into separate upstream-inspired destination pages.
- Standardized right-aligned controls and model-management actions.
- Improved application, taskbar, tray, and desktop-entry identity on Plasma Wayland.
- Suppressed one narrowly scoped GCC 16 diagnostic originating in Qt/generated CXX-Qt headers.

### Fixed

- PipeWire source selection, gain calculation, signal metering, and NaN display issues.
- Whisper capture normalization, short-result hallucinations, and automatic-language decoding.
- Global-shortcut startup ordering and lazy Wayland text-input permission handling.
- Tray visibility, stale single-instance locks, and second-launch window activation.
- QML startup errors, sidebar overlap, icon placement, and inconsistent action alignment.

## 0.1.0 - 2026-08-15

### Added

- Rust 2024 workspace and Qt Quick/CXX-Qt desktop shell.
- PipeWire microphone discovery and mono 16 kHz ASR conversion.
- Local whisper.cpp transcription with a CPU baseline.
- KDE Wayland global shortcut through the XDG portal.
- Clipboard recovery and direct-paste integration where the desktop permits it.
- Native Plasma tray integration and the first macOS-inspired recording overlay.
