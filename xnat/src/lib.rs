pub use oxinat_core::{
    Version,
    AdminUri,
    AuthUri,
    ServicesUri,
    UsersUri,
    Xnat,
    XnatBuilder,
    ClientCore,
    ClientAuth,
    ClientREST,
    ClientToken,
    ClientBuilderAttrs,
    ClientBuilderCore,
    ClientBuilderToken,
};
#[cfg(not(feature = "core"))]
use oxinat_core::FullUri;
#[cfg(feature = "core")]
pub extern crate oxinat_core;
#[cfg(feature = "core")]
pub use oxinat_core::*;
#[cfg(feature = "derive")]
pub extern crate oxinat_derive;
#[cfg(feature = "derive")]
pub use oxinat_derive::*;

#[derive(Clone, Version, AdminUri, AuthUri, ServicesUri, UsersUri)]
#[version(root_uri = "data", legacy = true)]
pub struct V1;
#[derive(Clone, Version, FullUri)]
#[version(root_uri = "xapi", data_uri = "data")]
pub struct V2;
