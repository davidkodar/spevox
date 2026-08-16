# Changelog

All notable changes to FluidVoice Linux are documented here. The project uses
[Semantic Versioning](https://semver.org/) while it remains in private preview.

## Unreleased

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
