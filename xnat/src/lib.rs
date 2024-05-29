use oxinat_core::*;

#[cfg(feature = "core")]
pub extern crate oxinat_core;
#[cfg(feature = "derive")]
pub extern crate oxinat_derive;

#[derive(Version, AdminUri, AuthUri, ServicesUri, UsersUri)]
#[version(root_uri = "data", legacy = true)]
pub struct V1;
#[derive(Version, FullUri)]
#[version(root_uri = "xapi", data_uri = "data")]
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
mod tests {
    use super::*;
    use oxinat_core::{UriBuilder, EventType, NotifyType, SubscriptionAction};

    #[test]
    fn test_version_v1_impls_admin_legacy01() {
        let ver = V1{};
        let uri = ver.config().build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), String::from("data/config"))
    }

    #[test]
    fn test_version_v2_impls_admin01() {
        let ver = V2{};
        assert_eq!(ver.site_config().build().unwrap(), String::from("xapi/siteConfig"));
        assert_eq!(ver.preferences().build().unwrap(), String::from("xapi/prefs"));
        assert_eq!(ver.schema().build().unwrap(), String::from("xapi/schemas"));
    }

    #[test]
    fn test_version_v2_impls_events_action01() {
        let ver = V2{};
        let uri = ver
            .events()
            .actions()
            .with_event_type(&EventType::One)
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/events/action")
    }

    #[test]
    fn test_version_v2_impls_events_action02() {
        let ver = V2{};
        let uri = ver
            .events()
            .actions()
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/events/allactions")
    }

    #[test]
    fn test_version_v2_impls_events_action03() {
        let ver = V2{};
        let uri = ver
            .events()
            .actions()
            .with_event_type(&EventType::Multiple)
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/events/actions");
    }

    #[test]
    fn test_version_v2_impls_events_subscription01() {
        let ver = V2{};
        let uri = ver
            .events()
            .subscription()
            .with_action(&SubscriptionAction::Filter)
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/events/subscription/filter");
    }

    #[test]
    fn test_version_v2_impls_events_subscription02() {
        let ver = V2{};
        let uri = ver
            .events()
            .subscription()
            .with_action(&SubscriptionAction::Validate)
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/events/subscription/validate");
    }

    #[test]
    fn test_version_v2_impls_events_subscription03() {
        let ver = V2{};
        let uri = ver
            .events()
            .subscription()
            .with_action(&SubscriptionAction::Activate)
            .with_id(&"SOME_ID")
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/events/subscription/SOME_ID/activate");
    }

    #[test]
    fn test_version_v2_impls_events_subscription04() {
        let ver = V2{};
        let uri = ver
            .events()
            .subscription()
            .with_id(&"SOME_ID")
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/events/subscription/SOME_ID");
    }

    #[test]
    fn test_version_v2_impls_site_config01() {
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
    fn test_version_v2_impls_sys01() {
        let ver = V2{};
        let uri = ver
            .archive()
            .catalogs()
            .refresh()
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/archive/catalogs/refresh");
    }

    #[test]
    fn test_version_v2_impls_sys02() {
        let ver = V2{};
        let uri = ver
            .archive()
            .catalogs()
            .refresh()
            .with_operations(&[
                    "delete".to_string(),
                    "append".to_string()
                ])
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/archive/catalogs/refresh/delete,append");
    }

    #[test]
    fn test_version_v2_impls_sys_notify01() {
        let ver = V2{};

        let nt = NotifyType::SmtpProperty(
            "auth".to_owned(),
            "HaHAhA".to_owned().into());
        let uri = ver
            .notifications()
            .notify()
            .with_notify_type(&nt)
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/notifications/notify/smtp/property/auth/HaHAhA");
    }

    #[test]
    fn test_version_v2_impls_sys_notify02() {
        let ver = V2{};
        let uri = ver
            .notifications()
            .notify()
            .with_notify_type(&NotifyType::Par)
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/notifications/notify/par")
    }

    #[test]
    fn test_version_v2_impls_users01() {
        let ver = V2{};
        let uri = ver.users().groups().build();
        assert!(uri.is_err(), "unset username must produce an error");
    }

    #[test]
    fn test_version_v2_impls_users02() {
        let ver = V2{};
        let uri = ver
            .users()
            .with_username(&"spyslikeus")
            .groups()
            .build();
        assert!(uri.is_ok(), "must be able to build without errors");
        assert_eq!(uri.unwrap(), "xapi/users/spyslikeus/groups");
    }
}
