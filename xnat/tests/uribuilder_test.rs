mod common;

use oxinat::*;
use oxinat_core::*;

macro_rules! assert_uri_eq {
    ($uri:ident, $pattern:literal) => {
        assert!($uri.is_ok(), "must be able to build without errors");
        assert_eq!($uri.unwrap(), $pattern, "URIs must match");
    };
}

#[test]
fn test_version_v1_impls_admin_legacy01() {
    let uri = V1.config().build();
    assert_uri_eq!(uri, "data/config");
}

#[test]
fn test_version_v2_impls_admin01() {
    assert_eq!(V2.site_config().build().unwrap(), "xapi/siteConfig");
    assert_eq!(V2.preferences().build().unwrap(), "xapi/prefs");
    assert_eq!(V2.schema().build().unwrap(), "xapi/schemas");
}

#[test]
fn test_version_v2_impls_events_action01() {
    let uri = V2
        .events()
        .actions()
        .with_event_type(EventType::One)
        .build();
    assert_uri_eq!(uri, "xapi/events/action");
}

#[test]
fn test_version_v2_impls_events_action02() {
    let uri = V2
        .events()
        .actions()
        .build();
    assert_uri_eq!(uri, "xapi/events/allactions");
}

#[test]
fn test_version_v2_impls_events_action03() {
    let uri = V2
        .events()
        .actions()
        .with_event_type(EventType::Multiple)
        .build();
    assert_uri_eq!(uri, "xapi/events/actions");
}

#[test]
fn test_version_v2_impls_events_subscription01() {
    let uri = V2
        .events()
        .subscription()
        .with_action(SubscriptionAction::Filter)
        .build();
    assert_uri_eq!(uri, "xapi/events/subscription/filter");
}

#[test]
fn test_version_v2_impls_events_subscription02() {
    let uri = V2
        .events()
        .subscription()
        .with_action(SubscriptionAction::Validate)
        .build();
    assert_uri_eq!(uri, "xapi/events/subscription/validate");
}

#[test]
fn test_version_v2_impls_events_subscription03() {
    let uri = V2
        .events()
        .subscription()
        .with_action(SubscriptionAction::Activate)
        .with_id("SOME_ID")
        .build();
    assert_uri_eq!(uri, "xapi/events/subscription/SOME_ID/activate");
}

#[test]
fn test_version_v2_impls_events_subscription04() {
    let uri = V2
        .events()
        .subscription()
        .with_id("SOME_ID")
        .build();
    assert_uri_eq!(uri, "xapi/events/subscription/SOME_ID");
}

#[test]
fn test_version_v2_impls_site_config01() {
    let uri = V2
        .site_config()
        .build_info()
        .with_property(&String::from("some_property"))
        .build();
    assert_uri_eq!(uri, "xapi/siteConfig/buildInfo/some_property");
}

#[test]
fn test_version_v2_impls_sys01() {
    let uri = V2
        .archive()
        .catalogs()
        .refresh()
        .build();
    assert_uri_eq!(uri, "xapi/archive/catalogs/refresh");
}

#[test]
fn test_version_v2_impls_sys02() {
    let uri = V2
        .archive()
        .catalogs()
        .refresh()
        .with_operations(&[
                "delete".to_string(),
                "append".to_string()
            ])
        .build();
    assert_uri_eq!(uri, "xapi/archive/catalogs/refresh/delete,append");
}

#[test]
fn test_version_v2_impls_sys_notify01() {
    let nt = NotifyType::SmtpProperty(
        "auth".to_owned(),
        "HaHAhA".to_owned().into());
    let uri = V2
        .notifications()
        .notify()
        .with_notify_type(nt)
        .build();
    assert_uri_eq!(uri, "xapi/notifications/notify/smtp/property/auth/HaHAhA");
}

#[test]
fn test_version_v2_impls_sys_notify02() {
    let uri = V2
        .notifications()
        .notify()
        .with_notify_type(NotifyType::Par)
        .build();
    assert_uri_eq!(uri, "xapi/notifications/notify/par");
}

#[test]
fn test_version_v2_impls_users01() {
    let uri = V2.users().groups().build();
    assert!(uri.is_err(), "unset username must produce an error");
}

#[test]
fn test_version_v2_impls_users02() {
    let uri = V2
        .users()
        .with_username("spyslikeus")
        .groups()
        .build();
    assert_uri_eq!(uri, "xapi/users/spyslikeus/groups");
}

#[test]
fn test_version_v2_impls_session_data01() {
    let uri = V2
        .experiment_data()
        .by_project("some_project")
        .build();
    assert_uri_eq!(uri, "data/projects/some_project/experiments");
}

#[test]
fn test_version_v2_impls_session_data02() {
    let uri = V2
        .experiment_data()
        .by_project("some_project")
        .with_experiment("some_session")
        .build();
    assert_uri_eq!(uri, "data/projects/some_project/experiments/some_session");
}

#[test]
fn test_version_v2_impls_session_data03() {
    let uri = V2
        .experiment_data()
        .by_project("some_project")
        .with_experiment("some_session")
        .scans()
        .with_scan("45")
        .build();
    assert_uri_eq!(uri, "data/projects/some_project/experiments/some_session/scans/45");
}
