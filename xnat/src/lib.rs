use oxinat_core::{AdminUri, AdminUriLegacy, SysUri, Version};

#[cfg(feature = "core")]
pub extern crate oxinat_core;
#[cfg(feature = "derive")]
pub extern crate oxinat_derive;

#[derive(Version, AdminUri)]
#[version(root_uri = "data", legacy = true)]
pub struct V1;
#[derive(Version, AdminUri, SysUri)]
#[version(root_uri = "xapi")]
pub struct V2;

// TODO: impl std::mem::Drop for this struct.
// https://stackoverflow.com/questions/42910662/is-it-possible-in-rust-to-delete-an-object-before-the-end-of-scope
#[derive(Clone, Debug)]
pub struct Xnat<V: Version> {
    hostname: String,
    username: Option<String>,
    password: Option<String>,
    version:  V,
}

pub struct XnatBuilder<V: Version> {
    hostname: String,
    username: Option<String>,
    password: Option<String>,
    version:  Option<V>,
}

#[cfg(test)]
mod test {
    use super::*;
    use oxinat_core::UriBuilder;

    #[test]
    fn test_version_v1_impls_admin_legacy() {
        let ver = V1{};
        let uri = ver.config().build();

        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), String::from("data/config"))
    }

    #[test]
    fn test_version_v2_impls_admin() {
        let ver = V2{};

        assert_eq!(ver.site_config().build().unwrap(), String::from("data/siteConfig"));
        assert_eq!(ver.preferences().build().unwrap(), String::from("data/prefs"));
        assert_eq!(ver.schema().build().unwrap(), String::from("data/schemas"));
    }

    #[test]
    fn test_version_v2_impls_site_config() {
        let ver = V2{};
        let uri = ver
            .site_config()
            .build_info()
            .with_property(&String::from("some_property"))
            .build();

        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/siteConfig/buildInfo/some_property")
    }
}
