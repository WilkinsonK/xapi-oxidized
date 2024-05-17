use std::borrow::Cow;

use oxinat_core::{AdminUri, AdminUriLegacy, Version};

#[cfg(feature = "core")]
pub extern crate oxinat_core;
#[cfg(feature = "derive")]
pub extern crate oxinat_derive;

#[derive(Version, AdminUri)]
#[version(root_uri = "data", legacy = true)]
pub struct V1;
#[derive(Version, AdminUri)]
#[version(root_uri = "xapi")]
pub struct V2;

#[derive(Clone, Debug)]
pub struct Xnat<'a, V: Version> {
    hostname: Cow<'a, str>,
    username: Option<Cow<'a, str>>,
    password: Option<Cow<'a, str>>,
    version:  V,
}

pub struct XnatBuilder<'a, V: Version> {
    hostname: Cow<'a, str>,
    username: Option<Cow<'a, str>>,
    password: Option<Cow<'a, str>>,
    version:  Option<V>,
}

#[cfg(test)]
mod test {
    use super::*;
    use oxinat_core::UriBuilder;

    #[test]
    fn test_version_v1_impls_admin() {
        let ver = V1{};

        assert_eq!(ver.site_config().build().unwrap(), String::from("data/siteConfig"));
        assert_eq!(ver.preferences().build().unwrap(), String::from("data/prefs"));
        assert_eq!(ver.schema().build().unwrap(), String::from("data/schemas"));
    }

    #[test]
    fn test_version_v1_impls_admin_legacy() {
        let ver = V1{};
        assert_eq!(ver.config(), String::from("data/config"))
    }
}
