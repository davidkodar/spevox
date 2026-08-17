# Changelog

All notable changes to FluidVoice Linux are documented here. The project uses
[Semantic Versioning](https://semver.org/) while it remains in private preview.

## 0.5.0 - 2026-08-17

- Changed the dictation overlay to dismiss after delivering text by default, so
  shortcut-driven dictation flows directly into the focused editor without a
  persistent interruption. Users can opt into keeping the final result popup.
- Replaced the result popup's permanent action-button row with a compact
  upstream-style actions menu for copy, raw recovery, AI undo, and reprocessing.

- Began the 0.5 intelligent-cleanup milestone with one conservative,
  language-aware editing contract for local and cloud providers. It explicitly
  preserves meaning and code-switching, applies punctuation and minimal repair,
  and forbids answering the dictation, translating it, or inventing content.
- Added bounded deterministic preprocessing before optional AI, model-quality
  and privacy guidance, and backward-compatible cleanup-mode/language metadata
  in History and JSON/CSV exports.
- Completed the five-phase structural, responsiveness, privacy, metadata, and
  regression-coverage milestone; the release gate is Clippy-clean, RustSec
  clean, and validates QML, AppStream, installation, and release packaging.

## 0.4.0 - 2026-08-16

- Began the post-0.4 structural milestone by replacing all seven controller
  `include!` sections with real Rust modules, explicit imports, and constrained
  `pub(super)` interfaces.
- Separated history persistence from Qt state mutation and extracted final ASR
  backend selection, native-engine fallback, and Whisper recovery behind plain
  Rust request/result boundaries.
- Split dictation capture, live preview, enhancement, persistence, delivery,
  and Qt presentation into focused stages; `toggle_recording` is now a small
  composition method without a Clippy line-count suppression.
- Moved Secret Service API-key writes and KWin automatic-profile configuration
  onto bounded workers so wallet prompts and desktop reconfiguration cannot
  freeze the Qt interface; stale completions are ignored after selection changes.
- Deferred history, lifetime-stat migration, dictionary, command history, AI
  profiles, retained-audio scanning, and Whisper model-state probing until the
  desktop-runtime worker, keeping controller construction free of those scans.
- Replaced per-dictation history rewrites with private append-first persistence;
  storage periodically compacts from 601 entries to the newest 500 while the UI
  consistently exposes the same 500-entry window.
- Derived PipeWire capture reservations and hard sample bounds from the
  negotiated rate/channel format, added core-level failure reporting, and
  limited Parakeet and Sortformer download updates to the same 50 ms UI cadence
  as Whisper downloads.
- Virtualized the searchable History results so large transcript collections
  create only the visible cards instead of eagerly instantiating every row.
- Consolidated dictation, AI retry, Write Mode, result recovery, and History
  copies behind one lazy clipboard helper; History actions now initialize the
  desktop clipboard themselves and preserve the underlying failure detail.
- Consolidated native runtime installation, automatic Vulkan-to-CPU fallback,
  model repair/download, and bounded progress publication for Parakeet and
  Sortformer setup workers.
- Replaced free-form AI-provider strings and internal numeric branches with
  typed provider identities and builder-style configurations while preserving
  the existing `0…9` settings-file and QML index mapping.
- Made each Write Mode job own its prompt, request input, paste policy, and
  retry-success messaging so draft and rewrite behavior share one definition.
- Centralized mono PCM16 WAV encoding across transcription, retained audio,
  meeting diarization, and diagnostics, with checked format bounds and one
  byte-exact test surface.
- Replaced the hand-written preferences serializer with a versioned JSON model;
  existing line-based files migrate transparently on their next save.
- Unified AI validation, loopback parsing, and timeout policy; authenticated
  local servers are supported, and non-idempotent enhancement POSTs are never
  retried after an ambiguous provider failure.
- Removed HOME-less private-state fallbacks into predictable shared temporary
  paths; startup now fails closed unless an XDG or home directory is available.
- Completed packaging hygiene by installing both high-resolution icon sizes,
  ignoring Flatpak build state, removing a vacuous portal-version branch, and
  adding a 44.1 kHz uneven-chunk streaming-resampler regression test.
- Closed follow-up audit regressions in meeting-file retry, local AI proxy
  isolation, IPv6 loopback ASR, Write Mode retry semantics, cancellable Whisper
  previews, model integrity checks, local API lifecycle handling, and native
  speech port selection. Cloud keyring reads no longer block the Qt thread.
- Moved one-time integrity migration for pre-marker Whisper downloads off the
  Qt thread. Read-only model directories now retain a fast length-based legacy
  fallback instead of re-hashing multi-gigabyte models on every use.
- Made the full Rust workspace Clippy-clean under `-D warnings`: removed
  redundant clones and conditions, completed HTTP test reads, centralized
  bounded UI/sample numeric conversions, tightened path APIs, and documented
  the few exact Qt/config/catalog boundaries where pedantic shape lints are
  intentional. CI now rejects every newly introduced Rust warning.
- Organized the oversized Qt controller into dedicated settings/profile,
  storage, dictionary, history, meeting, model-catalog, and speech-runtime
  source sections, plus independent update and Whisper-cache modules. The
  extraction preserves the existing CXX-Qt API while improving navigation; update
  comparison now also orders numeric prereleases correctly, and local AI test
  servers consume complete requests to eliminate flaky reset failures.
- Closed the remaining audit performance and architecture gaps: Whisper live
  preview and final decoding now reuse one bounded cached model context,
  inference thread counts follow available hardware, keyring access no longer
  blocks QML startup, history-derived UI data is cached, history mutations are
  serialized, real process arguments reach Qt, and the unused prototype core
  crate was removed in favor of accurate architecture documentation.
- Hardened remaining desktop integrations: local-API bearer tokens are no
  longer copied into Klipper history and the health route is now authenticated;
  portal consent uses a private persistent restore token, model-download
  progress is throttled, release artifacts no
  longer include an unused update manifest, and release binaries no longer
  search developer-checkout paths or use cwd-relative private data fallbacks.
- Replaced linear ASR downsampling with an optimized windowed-sinc filter that
  suppresses aliasing above the 8 kHz speech band, preserves phase across native
  realtime chunks, and reuses precomputed kernels. PipeWire device discovery is
  now time-bounded, and capture stream failures report their actual error.
- Made the AI master switch authoritative for Command Mode, Write Mode, retries,
  and provider tests; added persistent lifetime dictation counters that exclude
  imported files; corrected prerelease update comparisons, weekend streaks,
  oversized-file diagnostics, and final-ASR latency from preview-worker waits.
- Separated assistant, update, and export activity from actual transcription so
  unrelated background work no longer disables the dictation hotkey or presents
  misleading ASR state; reaped launched desktop/Ollama processes and corrected
  the diagnostics binary metadata, default message, and GUI exit handling.
- Hardened local security and failure handling: strict parsed loopback URLs,
  proxy-free/no-redirect audio requests, fail-closed atomic local-API tokens,
  bounded API connections, private atomic user data, private native-ASR ports,
  verified Whisper SHA-256 downloads, finite network timeouts, non-RT capture
  callbacks, immutable CI actions, and regression coverage for crash paths.
- Hardened the meeting workflow with retry-under-current-settings, visible last
  file, validated speaker renaming propagated to the current result, latest
  file History entry, and every export format. Added truthful compute
  diagnostics that distinguish requested Whisper GPU behavior from installed
  native CPU/Vulkan runtimes, accessible names for new controls, and a
  repeatable clean-install/release-candidate gate plus human QA checklist.
- Added optional experimental Sortformer v2 speaker diarization for file and
  meeting transcription: one-click shared-runtime/model setup, immutable
  revision and SHA-256 verification, CPU/cross-vendor Vulkan execution, up to
  four speaker labels, timestamp-overlap assignment, speaker-aware History and
  exports, clear capability guidance, cancellation/removal/repair controls,
  and non-destructive fallback to the complete Whisper transcript.
- Isolated the application-profile D-Bus integration test under a unique bus
  name with bounded waits, preventing the full validation suite from hanging
  when a real FluidVoice instance is already running during local release QA.
- Added a persistent four-step first-run onboarding wizard covering microphone
  and privacy expectations, practical Whisper speed/accuracy/size guidance,
  cross-vendor Vulkan behavior, experimental-engine limitations, and a final
  readiness check. Existing installations are not interrupted after upgrading,
  and the complete guide can be reopened from Getting Started.
- Disabled native realtime previews on the pinned Vulkan runtime after tracing
  real Nemotron audio to an upstream GGML tensor assertion. Vulkan remains the
  fast final-inference backend, while the stable Whisper overlay is used for
  live text; fallback activity is now labeled explicitly instead of appearing
  to be AI enhancement.
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
