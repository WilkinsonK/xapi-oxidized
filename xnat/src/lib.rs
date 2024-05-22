use oxinat_core::*;

#[cfg(feature = "core")]
pub extern crate oxinat_core;
#[cfg(feature = "derive")]
pub extern crate oxinat_derive;

#[derive(Version, AdminUri)]
#[version(root_uri = "data", legacy = true)]
pub struct V1;
#[derive(Version, AdminUri, SystemUri, UsersUri)]
#[version(root_uri = "xapi")]
pub struct V2;

// TODO: impl std::mem::Drop for this struct.
// https://stackoverflow.com/questions/42910662/is-it-possible-in-rust-to-delete-an-object-before-the-end-of-scope
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Xnat<V: Version> {
    hostname: String,
    username: Option<String>,
    password: Option<String>,
    version:  V,
}

#[allow(dead_code)]
pub struct XnatBuilder<V: Version> {
    hostname: String,
    username: Option<String>,
    password: Option<String>,
    version:  Option<V>,
}

#[cfg(test)]
mod test {
    use super::*;
    use oxinat_core::{UriBuilder, NotifyType};

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

        assert_eq!(ver.site_config().build().unwrap(), String::from("xapi/siteConfig"));
        assert_eq!(ver.preferences().build().unwrap(), String::from("xapi/prefs"));
        assert_eq!(ver.schema().build().unwrap(), String::from("xapi/schemas"));
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

    #[test]
    fn test_version_v2_impls_sys() {
        let ver = V2{};
        let partial_uri = ver
            .archive()
            .catalogs()
            .refresh();

        let uri = partial_uri.build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/archive/catalogs/refresh");

        let uri = partial_uri
            .with_operations(&[
                "delete".to_string(),
                "append".to_string()
            ])
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/archive/catalogs/refresh/delete,append");
    }

    #[test]
    fn test_version_v2_impls_sys_notify() {
        let ver = V2{};

        let partial_uri = ver
            .notifications()
            .notify();

        let nt = NotifyType::SmtpProperty(
            "auth".to_owned(),
            "HaHAhA".to_owned().into());
        let uri = partial_uri
            .clone()
            .with_notify_type(&nt)
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/notifications/notify/smtp/property/auth/HaHAhA");

        let uri = partial_uri
            .clone()
            .with_notify_type(&NotifyType::Par)
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/notifications/notify/par")
    }
}
