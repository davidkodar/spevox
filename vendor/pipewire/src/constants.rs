// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

//! Pipewire constants.

/// Invalid ID that matches any object when used for permissions.
// PipeWire defines PW_ID_ANY as (uint32_t)(0xffffffff). See the matching
// Clang 22 compatibility note in the vendored libspa constants.
pub const ID_ANY: u32 = u32::MAX;
