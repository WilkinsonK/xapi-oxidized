//! # Core functionality module for xapi. Defines structures and
//! methods that are general to the whole project, such as, error
//! handling and session building.
pub mod error;
pub mod session;
pub use session::{Session, NewSession, SessionCore, SessionMut, SessionREST};
