//! # Core functionality module for xapi. Defines structures and
//! methods that are general to the whole project, such as, error
//! handling and session building.
pub mod error;
mod resource;
pub use resource::{
    GetResource,
    ModResource,
    NewResource,
    Resource,
    Resources
};
mod status;
pub use status::{Status, StatusCode};
pub mod session;
pub use session::{
    NewSession,
    QueryArg,
    QueryArgs,
    QueryOpt,
    QueryOpts,
    Session,
    SessionCore,
    SessionMut,
    SessionREST,
    SessionQuery
};
