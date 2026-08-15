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

## Chunk-offset correction

Follow-up testing against Reaper revealed that Input 1 was known to be active while FluidVoice reported near-silence. The capture callback honored `spa_chunk.size` but initially ignored `spa_chunk.offset`, causing it to decode the wrong part of mapped PipeWire buffers. After correcting the slice to use both fields, Input 1 produced a peak of 0.0062 during a five-second capture. A regression test now covers non-zero PipeWire chunk offsets.

## Commands

```bash
cargo run -p fluidvoice-app -- --diagnose-audio 5
cargo run -p fluidvoice-app -- --diagnose-audio 5 alsa_input.usb-Focusrite_Scarlett_Solo_USB_XXXXXXXXXXXXXX-00.HiFi__Mic1__source
```
