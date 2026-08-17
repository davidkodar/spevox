# Post-0.4 Refactoring Milestone

## Goal

Pay down the structural and hygiene backlog identified after the three external
audit rounds without mixing those changes into the validated 0.4.0 defect
remediation. This milestone must preserve the existing KDE Wayland workflow,
settings compatibility, model files, history, and public QML behavior.

The work is intentionally ordered. Later phases should not begin until the
relevant earlier boundary is stable enough to make their changes smaller and
reviewable.

## Phase 1 — Real controller boundaries

Status: complete.

- Replace the controller's `include!` source sections with real Rust modules
  using explicit `pub(super)` APIs and imports.
- Keep Qt property mutation and signal emission in the CXX-Qt controller;
  helpers should accept and return plain Rust data.
- Split `toggle_recording` into capture setup, preview orchestration, final ASR,
  AI processing, and delivery operations.
- Remove justified `too_many_lines` allowances as functions become focused.
- Resolve the remaining Q16 controller tidy items while touching their owners.

Done when:

- `controller.rs` is a composition layer rather than the effective 6.5k-line
  compilation unit.
- Extracted modules compile independently through normal `mod` declarations.
- No extracted module requires `Pin<&mut ffi::FluidVoiceController>`.
- Dictation, cancellation, preview, retry, and clipboard recovery retain tests
  or deterministic seams.

## Phase 2 — Keep blocking work off the Qt thread

Status: complete.

- Move remaining synchronous `secret-tool` and KWin configuration processes
  (`save_ai_api_key`, profile-script configuration) to bounded workers.
- Move history loading/rewriting, statistics migration, model probing, audio
  summaries, profile loading, and other remaining startup filesystem work away
  from controller construction.
- Replace full history-file rewrites per dictation with an append/index or
  in-memory store whose persistence remains atomic and private.
- Derive capture allocation bounds from negotiated format and add a PipeWire
  core error listener.
- Throttle Parakeet and Sortformer progress updates to the same bounded UI rate
  used by Whisper downloads.
- Virtualize long history views instead of instantiating every row through a
  QML `Repeater`.

Done when:

- First-frame construction performs no model hashing, child-process waits, or
  unbounded history/model directory scans.
- Dictation does not synchronously rewrite the complete history on the Qt
  thread.
- Capture and native-model progress remain responsive under error and load.

## Phase 3 — Consolidate duplicated behavior

Status: completed for 0.4.0. Clipboard delivery, native runtime setup,
provider identity/configuration, Write Mode policy, PCM WAV encoding,
preferences migration, provider network policy, and enhancement validation now
each have a single implementation boundary.

- Introduce one lazy clipboard/delivery helper for all seven copy/paste paths.
- Consolidate native-runtime installation workers and progress handling.
- Replace throwaway `AiConfig` construction and numeric provider indexes with
  named constructors/identifiers.
- Give `WriteModeJob` ownership of its prompt and retry/delivery policy.
- Centralize PCM WAV encoding and remove the remaining hand-written loops.
- Replace the repeated preference representations and line parser with one
  versioned, migration-aware serialized settings model.
- Centralize provider timeouts and local-loopback URL policy.
- Make `enhance` and streaming enhancement share one validation path.

Done when:

- Each listed policy has one implementation and one focused test surface.
- Existing preference files migrate without losing user choices.
- No provider behavior depends on magic indexes such as Ollama's former `7`.

## Phase 4 — Integration and privacy hardening

Status: completed for 0.4.0. Local authentication is optional, ambiguous AI
POSTs are not retried, private state has no shared-temporary fallback, atomic
writes use owned create-new files, the session-bus trust boundary is documented,
and unsupported Flatpak integrations remain explicitly developer-preview.

- Make local-provider authentication optional but supported without weakening
  the local-only transport lock.
- Restrict AI POST retries to requests known not to have been processed, or
  require an idempotency mechanism where the provider supports one.
- Remove unsafe HOME-less shared-temporary fallbacks; fail closed or use a
  verified private runtime directory.
- Sweep only FluidVoice-owned stale atomic-write temporary files.
- Clarify and verify window-profile D-Bus ownership/sender policy.
- Gate unsupported Flatpak features at runtime or graduate the manifest from
  developer preview with portal-specific validation.
- Audit crate-level `unsafe_code = "allow"` and replace it with the narrowest
  possible boundary allowances and safety documentation.

Done when:

- Local-only requests cannot use a proxy, redirect, or unintended remote host,
  while authenticated loopback servers remain usable.
- No executable, token, or private state can be loaded from a shared predictable
  temporary path.
- Flatpak claims and runtime behavior match the actually tested package.

## Phase 5 — Metadata, assets, and regression coverage

Status: completed for 0.4.0. Metadata and installed assets are release-checked;
the migration, history-window, native-progress, model-integrity, and 44.1 kHz
streaming paths have focused regression coverage. Performance-sensitive startup,
history, model, and preview paths use bounded work or workers and are exercised
by the release gate.

- Correct the model-marker wording and any remaining user-facing capability
  claims.
- Decide whether to install/use the 512 px icon or remove the dead asset.
- Review `X-KDE-Wayland-VirtualKeyboard`, AppStream developer identity,
  screenshots, and release dates.
- Ignore `.flatpak-builder` and remove the vacuous shortcut-version check.
- Add missing tests for preference round-trips/migrations, history recording
  and pruning, audio-history budgets, lifetime-stat migration, and 44.1 kHz
  multi-phase streaming resampling.
- Add performance assertions or benchmarks for startup, history append, model
  switching, and preview-to-final handoff.

Done when:

- AppStream and desktop metadata validate and describe shipped behavior.
- The named missing regression tests exist and pass in CI.
- Strict Clippy, RustSec audit, QML lint, package installation, and the full
  release candidate check remain green.

## Change discipline

- Land each phase as small behavior-preserving commits; do not combine the
  controller extraction with feature work.
- Preserve sole authorship configuration and never commit local audit reports.
- Run focused tests during extraction and the full release gate at the end of
  every phase.
- If a refactor reveals a functional or security defect, fix it in an isolated
  commit and add a regression test before resuming structural work.

## Non-goals

- Reproducing the closed Fluid Intelligence model.
- Replacing Qt/QML, PipeWire, portals, Whisper, or the native NeMo runtime.
- Changing public shortcuts, persisted settings, model locations, or history
  formats without an explicit migration.
- Delaying the already validated 0.4.0 candidate solely for cosmetic cleanup.
