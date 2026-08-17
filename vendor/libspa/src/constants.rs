// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

//! libspa constants.

/// Invalid ID intended for errors or as a sentinel.
// PipeWire defines SPA_ID_INVALID as ((uint32_t)0xffffffff). Bindgen's Clang
// fallback currently omits cast-style macros with Clang 22, so spell out the
// exact public ABI value instead of depending on that generated constant.
pub const ID_INVALID: u32 = u32::MAX;
