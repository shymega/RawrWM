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

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_os = "windows")]
async fn windows_main() -> Result<(), Error> {
    Ok(())
}

#[cfg(target_os = "macos")]
async fn macos_main() -> Result<(), Error> {
    Ok(())
}

#[cfg(target_os = "linux")]
async fn linux_main() -> Result<(), Error> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    #[cfg(target_os = "windows")]
    windows_main().await?;

    #[cfg(target_os = "macos")]
    macos_main().await?;

    #[cfg(target_os = "linux")]
    linux_main().await?;

    Ok(())
}
