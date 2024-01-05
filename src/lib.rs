//! # XAPI Oxidized
//! Interacts with a remote XNAT via REST exposing the **XAPI** as
//! bindings in Rust.
// Definitions exposed by the API should be
// managed by the individual modules, not the
// library root. If an artifact is exposed as
// **public** by the module, it will be public
// from `xapi_oxidized`.
pub mod core;
pub use core::*;
pub mod xapi;
pub use xapi::*;
