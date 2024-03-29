// SPDX-FileCopyrightText: 2023 The RawrWM Developers
//
// SPDX-License-Identifier: Apache-2.0

//! Core (shared) crate for `RawrWM`.
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

pub mod reexports {
    //! Re-exports of RawrWM components.
    pub use anyhow;
    pub use dirs;
    pub use jsonrpc;
    pub use jsonrpc_core_client;
    pub use jsonrpc_ipc_server;
    pub use rawrwm_backends as backends;
    pub use serde;
    pub use thiserror;
    pub use tokio;
}
