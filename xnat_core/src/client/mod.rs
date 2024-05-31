pub mod builder;
pub mod clients;
pub mod error;
pub mod timeouts;

pub static APP_CONNECT_TIMEOUT: u64 = 5;
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION")
);

pub use builder::{
    ClientBuilderCore,
    ClientBuilderAttrs,
    ClientBuilderToken,
    XnatBuilder
};
pub use clients::{
    ClientAuth,
    ClientCore,
    ClientToken,
    Xnat
};
