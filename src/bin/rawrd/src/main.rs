// SPDX-FileCopyrightText: 2023 The RawrWM Developers
//
// SPDX-License-Identifier: Apache-2.0

//! Daemon for `RawrWM`
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

pub use rawrwm::reexports::anyhow::{Context, Result};
pub use rawrwm::reexports::thiserror::Error;
pub use rawrwm_core as rawrwm;

#[derive(Error, Debug)]
#[allow(dead_code)]
enum AppStartError {
    #[error("Initialization error")]
    InitializationError,
}

type AppStartResult = Result<(), AppStartError>;

#[cfg(target_os = "windows")]
async fn windows_main() -> AppStartResult {
    Ok(())
}

#[cfg(target_os = "macos")]
async fn macos_main() -> AppStartResult {
    Ok(())
}

#[cfg(all(
    target_family = "unix",
    not(any(target_os = "ios", target_os = "macos"))
))]
async fn unix_main() -> AppStartResult {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(target_os = "windows")]
    windows_main().await.context("Initialization error?")?;

    #[cfg(target_os = "macos")]
    macos_main().await.context("Initialization error?")?;

    #[cfg(all(
        target_family = "unix",
        not(any(target_os = "ios", target_os = "macos"))
    ))]
    unix_main().await.context("Initialization error?")?;

    Ok(())
}
