# Third-party notices

Spevox is an independent Linux application authored and maintained by David
Bolin. The original FluidVoice macOS project is the work of Altic and the
upstream FluidVoice contributors. Spevox is not sponsored or endorsed by them.

## FluidVoice artwork

Current Spevox releases use original branding and do not distribute FluidVoice
icons. Private preview versions through 0.5.x included copies of the application
and menu-bar icons from the upstream FluidVoice project. Their provenance is
retained here as a historical compliance record:

- Source: https://github.com/altic-dev/FluidVoice
- Upstream revision: `ac9b3ad3cef4a669bf8a1345f313bbc57e5b1442`
- Revision date: 2026-08-16
- License file at that revision: GPL-3.0 (`f288702d2fa16d3cdf0035b15a9fcbc552cd88e7` Git blob)
- Original paths:
  - `Sources/Fluid/Assets.xcassets/AppIcon.appiconset/`
  - `Sources/Fluid/Assets.xcassets/MenuBarIcon.imageset/`
- Copyright: the FluidVoice contributors
- License: GNU General Public License, version 3

Those historical assets were redistributed in the private preview under GPLv3.
FluidVoice and its artwork may identify the upstream project; this project does
not claim sponsorship, endorsement, or a trademark license from Altic or the
upstream maintainers. The copied files match these upstream Git blobs:

- 256 px application icon: `34864dc292dbd56417f6feb963c7ed62050f64bf`
- 512 px application icon: `c1d8af8a72808c87d7ec16c5b520702eeee68444`
- 3x menu-bar/tray icon: `c7364d2c0f7be55180ceba1dd4759d723d781e1c`

The complete GPLv3 license is provided in [`LICENSE`](LICENSE).

## whisper.cpp and ggml

The default speech engine statically links whisper.cpp and ggml through the
`whisper-rs`/`whisper-rs-sys` crates.

- Source: https://github.com/ggml-org/whisper.cpp
- Copyright (c) 2023-2024 The ggml authors
- License: MIT

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

The downloadable Whisper GGML models (`ggml-*.bin`) are converted from
OpenAI Whisper and distributed under the MIT License by the whisper.cpp
project.

## Qt

The desktop interface dynamically links Qt 6 (Core, Gui, Qml, Quick,
QuickControls2, Network, Widgets), which is available under the GNU Lesser
General Public License version 3. Spevox itself is GPLv3, and the Qt
libraries are provided by the host system or distribution package. See
https://www.qt.io/licensing/ for details.

## NVIDIA NeMo-Speech.cpp

Spevox can optionally download and build the native NeMo-Speech.cpp runtime.
It is not linked into the Spevox executable.

- Source: https://github.com/NVIDIA/NeMo-Speech.cpp
- Pinned revision: `9bc876635af36df537d9bc6d3f57ad1b76e4f74a`
- Copyright: NVIDIA Corporation and contributors
- License: Apache License 2.0

## NVIDIA Parakeet TDT 0.6B v3 model

The optional quantized multilingual model is downloaded separately at the
user's request and is never redistributed in the Spevox source tree.

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
after the user selects setup. The GGUF is a Spevox conversion of NVIDIA's
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
