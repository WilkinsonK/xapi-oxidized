//! Interacts with a remote XNAT via REST exposing the **XAPI** as
//! bindings in Rust.
pub mod error;
pub mod session;
pub use session::{Session, NewSession, SessionCore, SessionMut, SessionREST};
