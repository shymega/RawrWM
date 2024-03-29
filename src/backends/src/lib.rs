// SPDX-FileCopyrightText: 2023 The RawrWM Developers
//
// SPDX-License-Identifier: Apache-2.0

//! Conditionally-compiled, all backends crate for `RawrWM`.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

#[cfg_attr(
    all(
        target_family = "unix",
        not(any(target_os = "ios", target_os = "macos"))
    ),
    path = "platforms/unix/mod.rs"
)]
#[cfg_attr(target_os = "windows", path = "platforms/windows/mod.rs")]
#[cfg_attr(target_os = "macos", path = "platforms/macos/mod.rs")]
pub mod platform;

pub mod common;
