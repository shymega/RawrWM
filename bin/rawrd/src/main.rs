#![deny(
//    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
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

use std::io::Error;

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
enum AppStartError {
    #[error("Initialization error")]
    InitializationError,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_os = "windows")]
async fn windows_main() -> Result<(), AppStartError> {
    Ok(())
}

#[cfg(target_os = "macos")]
async fn macos_main() -> Result<(), AppStartError> {
    Ok(())
}

#[cfg(target_os = "linux")]
async fn linux_main() -> Result<(), AppStartError> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(target_os = "windows")]
    windows_main().await.context("Initialization error?")?;

    #[cfg(target_os = "macos")]
    macos_main().await.context("Initialization error?")?;

    #[cfg(target_os = "linux")]
    linux_main().await.context("Initialization error?")?;

    Ok(())
}
