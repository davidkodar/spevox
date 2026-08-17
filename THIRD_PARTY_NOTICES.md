# Third-party notices

FluidVoice Linux is an independent Linux implementation authored and maintained
by David Bolin. The original FluidVoice macOS project and the artwork identified
below are the work of Altic and the upstream FluidVoice contributors. This port
is not sponsored or endorsed by them.

## FluidVoice artwork

This unofficial Linux port includes copies of the application icon and menu-bar
icon from the upstream FluidVoice project:

- Source: https://github.com/altic-dev/FluidVoice
- Upstream revision: `ac9b3ad3cef4a669bf8a1345f313bbc57e5b1442`
- Revision date: 2026-08-16
- License file at that revision: GPL-3.0 (`f288702d2fa16d3cdf0035b15a9fcbc552cd88e7` Git blob)
- Original paths:
  - `Sources/Fluid/Assets.xcassets/AppIcon.appiconset/`
  - `Sources/Fluid/Assets.xcassets/MenuBarIcon.imageset/`
- Copyright: the FluidVoice contributors
- License: GNU General Public License, version 3

The assets are redistributed for use by this modified, unofficial Linux port.
FluidVoice and its artwork may identify the upstream project; this project does
not claim sponsorship, endorsement, or a trademark license from Altic or the
upstream maintainers. The copied files match these upstream Git blobs:

- 256 px application icon: `34864dc292dbd56417f6feb963c7ed62050f64bf`
- 512 px application icon: `c1d8af8a72808c87d7ec16c5b520702eeee68444`
- 3x menu-bar/tray icon: `c7364d2c0f7be55180ceba1dd4759d723d781e1c`

The complete GPLv3 license is provided in [`LICENSE`](LICENSE).

## NVIDIA NeMo-Speech.cpp

FluidVoice can optionally download and build the native NeMo-Speech.cpp
runtime. It is not linked into the FluidVoice executable.

- Source: https://github.com/NVIDIA/NeMo-Speech.cpp
- Pinned revision: `9bc876635af36df537d9bc6d3f57ad1b76e4f74a`
- Copyright: NVIDIA Corporation and contributors
- License: Apache License 2.0

## NVIDIA Parakeet TDT 0.6B v3 model

The optional quantized multilingual model is downloaded separately at the
user's request and is never redistributed in the FluidVoice source tree.

- Source: https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3
- File: `parakeet-tdt-0.6b-v3.q8_0.gguf`
- Copyright: NVIDIA Corporation
- License: Creative Commons Attribution 4.0 International (CC BY 4.0)

## Additional NVIDIA speech models

The following optional quantized models are downloaded separately only after
the user selects Install; they are not redistributed in this source tree.

- Nemotron 3.5 ASR Streaming 0.6B
  - Source: https://huggingface.co/nvidia/nemotron-3.5-asr-streaming-0.6b
  - File: `nemotron-3.5-asr-streaming-0.6b.q8_0.gguf`
  - License: NVIDIA Open Model Development and Work License 1.1
- Nemotron Speech Streaming English 0.6B
  - Source: https://huggingface.co/nvidia/nemotron-speech-streaming-en-0.6b
  - File: `nemotron-speech-streaming-en-0.6b.q8_0.gguf`
  - License: NVIDIA Open Model License
- Parakeet CTC 1.1B
  - Source: https://huggingface.co/nvidia/parakeet-ctc-1.1b
  - File: `parakeet-ctc-1.1b.q8_0.gguf`
  - License: Creative Commons Attribution 4.0 International (CC BY 4.0)

Copyright for these models is held by NVIDIA Corporation.

## Sortformer speaker diarization model

The optional experimental diarization model is downloaded separately only
after the user selects setup. The GGUF is a FluidVoice conversion of NVIDIA's
public Sortformer 4-speaker v2 checkpoint and is not presented as an
NVIDIA-published binary. It is created with the converter from the pinned
NeMo-Speech.cpp source revision listed above.

- Original model: https://huggingface.co/nvidia/diar_streaming_sortformer_4spk-v2
- Pinned model revision: `5240a64075176943f677d30fa2171c780229f341`
- Original checkpoint SHA-256: `b371afce2c4958186469df33d939936b9746c89f38b10a69cfd2c61254e83329`
- File: `sortformer-v2-q8_0.gguf`
- Converted file SHA-256: `0679cfeb1ce356d0dea9470b31274f4bfc7eb927497d82005483770666da998a`
- Copyright: NVIDIA Corporation
- License: Creative Commons Attribution 4.0 International (CC BY 4.0)

## SentencePiece

The optional managed Parakeet runtime builds a pinned private SentencePiece
archive using the NeMo-Speech.cpp dependency script.

- Source: https://github.com/google/sentencepiece
- Pinned revision: `17d7580d6407802f85855d2cc9190634e2c95624`
- Copyright: Google Inc. and contributors
- License: Apache License 2.0
