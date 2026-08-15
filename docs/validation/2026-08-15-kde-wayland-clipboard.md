# KDE Wayland clipboard recovery validation — 2026-08-15

## Implementation

- Backend: `arboard` 3.6.1
- Native Wayland support: `wayland-data-control` enabled
- Selection: standard clipboard, not primary selection
- Ownership: application-lifetime `ClipboardDelivery` instance
- Safety: empty transcripts are rejected, and every write is immediately read back before success is reported

Keeping the owner alive is required because Linux Wayland and X11 clipboards are served by the process that owns the selection. KDE Klipper may retain clipboard history, but the application does not rely on that behavior for correctness.

## Live result

The diagnostic copied:

> FluidVoice Linux clipboard roundtrip 2026-08-15

The application read-back matched exactly. While the application still owned the selection, KDE Klipper independently returned the same text through `org.kde.klipper.klipper.getClipboardContents`.

## Integrated workflow

The application now implements this diagnostic path:

```text
KDE shortcut activation
    → bounded PipeWire capture
    → mono 16 kHz normalization
    → local whisper.cpp transcription
    → reject empty transcript
    → copy and verify clipboard text
    → remain alive to serve the Wayland selection
```

The integrated executable compiled and waited successfully for the portal event. No physical activation arrived during the one-minute interactive validation window, so a combined shortcut-to-clipboard live run remains pending. The shortcut, capture, transcription, and clipboard boundaries have each been live-validated separately.
