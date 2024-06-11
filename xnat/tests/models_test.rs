mod common;

use oxinat::{ClientCore, ClientToken};
use oxinat_core::{
    models::{Items, Project, ResultSet, SiteConfig},
    AdminUri,
    ClientREST
};

#[test]
fn test_models_items01() {
    common::init();

    let data = r#"{
        "items": [
            {
                "children": [],
                "meta": {
                    "create_event_id": 0,
                    "xsi:type": "xnat:projectData",
                    "isHistory": true,
                    "start_date": "Fri Sep 29 10:18:24 CDT 1989"
                },
                "data_fields": {
                    "secondary_ID": "BBH_STARFISH",
                    "name": "BBH_STARFISH",
                    "ID": "BBH_STARFISH"
                }
            }
        ]
    }"#;

    let parsed = serde_json::from_str::<Items<Project>>(data);
    assert!(parsed.is_ok(), "must be able to deserialize from JSON: {parsed:?}");

    let from_parsed = &parsed
        .unwrap()
        .items[0]
        .data_fields;

    let from_parsed = from_parsed
        .clone()
        .unwrap()
        .name
        .unwrap();
    assert_eq!(from_parsed, "BBH_STARFISH");
}

#[test]
fn test_models_project01() {
    common::init();

    let data = r#"
    {
        "pi_firstname": "Gill",
        "secondary_ID": "",
        "pi_lastname": "Gilliam",
        "name": "Bikini Bottom Hospital",
        "description": "",
        "ID": "",
        "URI": ""
    }"#;
    let parsed = serde_json::from_str::<Project>(data);
    assert!(parsed.is_ok(), "must be able to deserialize from JSON: {parsed:?}");
    assert!(parsed.unwrap().name.is_some_and(|n| n == "Bikini Bottom Hospital"));
}

#[test]
fn test_models_resultset01() {
    let data = r#"{
        "ResultSet": {
            "Result": [
                "metadata",
                "tasks",
                "volatile"
            ],
            "Columns": "No columns in an array",
            "title": "String Array",
            "totalRecords": 100
        }
    }"#;
    let parsed = serde_json::from_str::<ResultSet<String>>(data);
    assert!(parsed.is_ok(), "must be able to deserialize from JSON: {parsed:?}");
    assert!(parsed.unwrap().data.result[1] == "tasks")
}

#[test]
fn test_models_resultset02() {
    let data = r#"{
        "ResultSet": {
            "Result": [
                {
                    "pi_firstname": "Gill",
                    "secondary_ID": "",
                    "pi_lastname": "Gilliam",
                    "name": "Bikini Bottom Hospital",
                    "description": "",
                    "ID": "",
                    "URI": ""
                }
            ]
        }
    }"#;
    let parsed = serde_json::from_str::<ResultSet<Project>>(data);
    assert!(parsed.is_ok(), "must be able to deserialize from JSON: {parsed:?}");
}

#[tokio::test]
#[ignore = "must have a .env file or variables set in env"]
async fn test_models_siteconfig_get_raw() {
    common::init();
    let mut client = common::request_client().await;

    let res = client
        .get(&client.version().site_config())
        .await
        .unwrap()
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    client.release().await.unwrap();
}

#[tokio::test]
#[ignore = "must have a .env file or variables set in env"]
async fn test_models_siteconfig_preference01() {
    use oxinat_core::models::admin::siteconfig::AdminEmail;

    common::init();
    let mut client = common::request_client().await;

    let res = client
        .get(&client.version().site_config().values("adminEmail"))
        .await
        .unwrap()
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    let pref = res.json::<AdminEmail>().await;
    assert!(pref.is_ok(), "must be able to deserialize a preference: {pref:?}");

    client.release().await.unwrap();
}

#[tokio::test]
#[ignore = "must have a .env file or variables set in env"]
async fn test_models_siteconfig_preference02() {
    use oxinat_core::models::admin::siteconfig::UiDisplaySeriesDescription;

    common::init();
    let mut client = common::request_client().await;

    let res = client
        .get(&client.version().site_config().values("uiDisplaySeriesDescription"))
        .await
        .unwrap()
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    let pref = res.json::<UiDisplaySeriesDescription>().await;
    assert!(pref.is_ok(), "must be able to deserialize a preference");

    client.release().await.unwrap();
}

#[tokio::test]
#[ignore = "must have a .env file or variables set in env"]
async fn test_models_siteconfig01() {
    common::init();
    let mut client = common::request_client().await;

    let res = client
        .get(&client.version().site_config())
        .await
        .unwrap()
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    let site_config = res.json::<SiteConfig>().await;
    assert!(site_config.is_ok(), "must be able to deserialize a siteConfig: {site_config:?}");

    let site_config = site_config.unwrap();
    let enable_csrf_token = site_config.enable_csrf_token;
    assert!(enable_csrf_token.is_some());

    client.release().await.unwrap();
}
