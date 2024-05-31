use oxinat_core::*;

#[cfg(feature = "core")]
pub extern crate oxinat_core;
#[cfg(feature = "derive")]
pub extern crate oxinat_derive;

#[derive(Clone, Version, AdminUri, AuthUri, ServicesUri, UsersUri)]
#[version(root_uri = "data", legacy = true)]
pub struct V1;
#[derive(Clone, Version, FullUri)]
#[version(root_uri = "xapi", data_uri = "data")]
pub struct V2;
