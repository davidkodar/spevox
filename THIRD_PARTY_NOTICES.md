# Third-party notices

## FluidVoice artwork

This unofficial Linux port includes copies of the application icon and menu-bar
icon from the upstream FluidVoice project:

- Source: https://github.com/altic-dev/FluidVoice
- Upstream revision: `ac9b3ad3cef4a669bf8a1345f313bbc57e5b1442`
- Original paths:
  - `Sources/Fluid/Assets.xcassets/AppIcon.appiconset/`
  - `Sources/Fluid/Assets.xcassets/MenuBarIcon.imageset/`
- Copyright: the FluidVoice contributors
- License: GNU General Public License, version 3

The assets are redistributed for use by this modified, unofficial Linux port.
FluidVoice and its artwork may identify the upstream project; this project does
not claim sponsorship or endorsement by Altic or the upstream maintainers.

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
