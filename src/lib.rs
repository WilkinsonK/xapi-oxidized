//! # Oxidized XNAT REST
//! Interacts with a remote XNAT via REST exposing the **XAPI** as
//! bindings in Rust.
pub mod core;
pub use core::*;
pub mod xapi;
