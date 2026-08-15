# PipeWire microphone capture validation — 2026-08-15

## Environment

- KDE Plasma on Wayland
- PipeWire 1.6.8
- Audio interface: Focusrite Scarlett Solo (3rd Gen.)
- Native negotiated format: F32LE, 48 kHz, mono

## Results

The diagnostic discovered both exposed hardware sources:

- Input 1 Mic
- Input 2 Inst/Line

A five-second capture from the desktop default source returned 238,848 frames (4.98 seconds). An explicit capture from Input 1 Mic returned 239,616 frames (4.99 seconds). Neither capture overflowed the preallocated realtime buffer.

Both native captures converted successfully to the application ASR contract: mono F32 samples at 16 kHz. The explicit Input 1 capture produced 79,872 normalized samples with the same 4.99-second duration.

Signal levels were near silence during validation (peak 0.0002 or lower), so this validates graph connection, source selection, bounded capture, negotiated-format handling, and resampling—not microphone gain or speech intelligibility. Speech recognition validation belongs to the transcription milestone.

## Commands

```bash
cargo run -p fluidvoice-app -- --diagnose-audio 5
cargo run -p fluidvoice-app -- --diagnose-audio 5 alsa_input.usb-Focusrite_Scarlett_Solo_USB_XXXXXXXXXXXXXX-00.HiFi__Mic1__source
```
