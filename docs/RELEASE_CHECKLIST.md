# Spevox release checklist

## Automated gate

- Work from a clean `main` worktree with the intended version and changelog.
- Run `./packaging/release-check.sh` on KDE Plasma Wayland.
- Confirm the locked workspace tests, release build, QML validation, AppStream
  metadata, dependency-license policy and bundle, Flatpak YAML, staged
  installation, archive contents, portable checksum, and build-path redaction
  all pass.
- Build optional model assets only with their documented pinned conversion
  scripts; compare the generated SHA-256 with the application catalog.

## Fresh-user acceptance

- Install into a disposable user or VM with no existing Spevox config or
  data directory. Confirm onboarding appears exactly once and can be reopened.
- Verify the application icon, task manager entry, tray icon, single-instance
  activation, close-to-tray behavior, and explicit Quit action.
- Select and test a PipeWire microphone; confirm the meter, input selection,
  software gain, hold shortcut, live overlay, final transcript, and Wayland
  paste/clipboard recovery.
- Download one Whisper model and confirm CPU-only and Automatic modes. Treat
  “GPU requested” as distinct from a managed native Vulkan runtime being
  installed; do not claim an active backend without evidence.
- Restart the application and confirm microphone, model, shortcut, overlay,
  theme, AI master switch, compute, and diarization preferences persist.

## Optional feature acceptance

- Test a native ASR model with CPU and Vulkan where available; force one model
  failure and confirm the Whisper fallback retains the recording.
- Test Sortformer on a real two-speaker recording. Confirm speaker labels,
  renaming, History, TXT/Markdown/SRT/VTT/JSON exports, and fallback without
  labels when diarization is unavailable.
- Test local AI with enhancement disabled and enabled. Disabled must never
  contact the provider or display AI-only processing states.
- Exercise History deletion/export, optional audio retention, dictionary
  import/export, statistics, Write Mode, and the local API opt-in boundary.

## UI and accessibility

- Test the minimum supported window size and 100%, 125%, 150%, and 200% Plasma
  scaling. No text or control may overlap, clip outside its card, or become
  unreachable by scrolling.
- Navigate all interactive controls with the keyboard and verify visible focus.
- Check accessible names for icon-only or context-dependent actions and ensure
  status changes are also represented as text, not color alone.
- Review dark, light, and system themes with KDE system and explicit accents.

## Publish

- Confirm `CREDITS.md`, `THIRD_PARTY_NOTICES.md`,
  `THIRD_PARTY_LICENSES.html`, README feature parity, limitations, model
  licenses, and `CHANGELOG.md` match the actual build.
- Run Gitleaks against all refs and confirm no credentials or private data exist
  anywhere in Git history.
- Confirm copied upstream artwork matches the pinned Git blobs and that the
  license at the pinned revision is still documented accurately.
- Create the release from the tested commit, upload checksummed artifacts and,
  when the automated release workflow is used, Sigstore signatures. Download
  the published artifacts into a new directory and verify them by basename.
- Confirm the release archive contains corresponding source, credit, notices,
  dependency licenses, and no local build paths before announcing it.
- For the first public release, enable deletion and force-push protection on
  `main`, private vulnerability reporting, and GitHub secret scanning as soon as
  repository visibility changes.
