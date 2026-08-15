# FluidVoice Linux

Native, local-first voice dictation for KDE Plasma on Wayland.

This private repository contains the Linux client project. It is currently in the foundation and technical-spike phase; it is not ready for use or redistribution.

## First milestone

Prove the complete local workflow:

```text
KDE global shortcut
    → PipeWire microphone capture
    → whisper.cpp transcription
    → clipboard recovery
```

Reliable automatic insertion into Wayland applications is a separate validation milestone. The implementation will use capability detection and will not require root access or weakened Wayland security by default.

## Initial technology choices

- C++20 and CMake
- Qt 6 for the KDE-native application shell and D-Bus integration
- PipeWire for microphone capture
- XDG Global Shortcuts portal for hold/toggle activation
- whisper.cpp with a CPU baseline and optional acceleration
- AT-SPI and consented Wayland interfaces for text-delivery research

## Build the foundation

Requirements:

- CMake 3.24 or newer
- A C++20 compiler
- Qt 6.6 or newer with Core, DBus, and Widgets

```bash
cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug
cmake --build build --parallel
./build/fluidvoice-linux
```

## Status

The current executable only establishes the Qt application, settings placeholder, and Plasma system tray lifecycle. Audio, shortcuts, transcription, and text delivery have not yet been implemented.

## Upstream relationship and licensing

The project is informed by the GPLv3-licensed [FluidVoice](https://github.com/altic-dev/FluidVoice) macOS application. Before the first redistribution, this repository will include the complete license and attribution inventory for all copied or adapted upstream material, dependencies, and speech models.

Fluid Intelligence is a separate private component and is not part of this project.

