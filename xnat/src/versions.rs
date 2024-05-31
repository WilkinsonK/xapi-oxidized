use oxinat_core::*;

#[derive(Clone, Version, AdminUri, AuthUri, ServicesUri, UsersUri)]
#[version(root_uri = "data", legacy = true)]
pub struct V1;
#[derive(Clone, Version, FullUri)]
#[version(root_uri = "xapi", data_uri = "data")]
pub struct V2;
