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

> Spevox clipboard roundtrip 2026-08-15

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

Physical activation and deactivation were subsequently received from KDE. Activation started PipeWire capture and release stopped it at 122.69 ms, producing 1,963 normalized 16 kHz samples rather than running to the 30-second safety ceiling. A 300 ms accidental-tap guard now prevents such very short captures from entering inference.

The earlier longer physical run also reached local inference, but the connected microphone input supplied near-silence and Whisper correctly produced an empty transcript. The safety rule left the clipboard unchanged. A combined run that produces and copies non-empty live speech remains pending on an active microphone signal; release-driven recording itself is validated.
