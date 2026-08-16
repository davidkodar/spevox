# Changelog

All notable changes to FluidVoice Linux are documented here. The project uses
[Semantic Versioning](https://semver.org/) while it remains in private preview.

## Unreleased

- Added optional post-transcription AI cleanup for OpenAI, Anthropic, xAI,
  Groq, Cerebras, Gemini, OpenRouter, Ollama, LM Studio, and custom
  OpenAI-compatible endpoints.
- Added secure API-key storage through the desktop Secret Service, editable
  cleanup prompts, provider verification, and raw-transcript fallback.
- Added Ollama and LM Studio model discovery, installed-model selection, a
  fully-local privacy indicator, and loopback-only enforcement for local presets.
- Added a persistent local-provider privacy lock, enabled by default, plus
  bounded provider responses and retries for transient failures.
- Broader file-format support beyond 16-bit PCM WAV.
- Added searchable history cards with relative and full local timestamps and word counts.
- Expanded local statistics with today's activity, estimated time saved, streaks,
  average session size, and a seven-day activity chart.
- Richer history metadata, audio retention, and command-mode workflows are planned.

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
