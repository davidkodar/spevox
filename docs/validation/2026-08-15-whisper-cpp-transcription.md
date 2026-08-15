# Local whisper.cpp transcription validation — 2026-08-15

## Implementation

- Rust binding: `whisper-rs` 0.16.0
- Backend: bundled `whisper.cpp`, CPU-only baseline
- Input contract: mono F32 at 16 kHz
- Sampling: greedy, one decoder
- Language: automatic detection by default; fixed ISO 639-1 language supported
- Context: disabled between independent dictation captures
- Worker threads: detected automatically and capped at eight

The model remains external data and is not committed to the repository.

## Model

- Official multilingual tiny model: `ggml-tiny.bin`
- Size: approximately 75 MiB
- SHA-1: `bd577a113a864445d4c299885e0cb97d4ba92b5f`

## Results

Live Input 1 capture produced 111,531 normalized samples from seven seconds of PipeWire audio. The model loaded, inference completed, and English was auto-detected. The connected microphone signal was silent, so the transcript was correctly empty; this does not validate live speech intelligibility.

Recognition was independently validated using the official 16 kHz mono JFK sample from `whisper.cpp`. The local tiny model returned:

> And so my fellow Americans ask not what your country can do for you, ask what you can do for your country.

No remote transcription API or network request was used during inference.

## Commands

```bash
cargo run -p fluidvoice-app -- --diagnose-transcription work/models/ggml-tiny.bin 7 PIPEWIRE_NODE
cargo run -p fluidvoice-app -- --diagnose-transcription-file work/models/ggml-tiny.bin work/jfk.wav
```
