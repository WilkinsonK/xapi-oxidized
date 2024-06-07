mod common;

use oxinat::{ClientCore, ClientToken};
use oxinat_core::{models::SiteConfig, AdminUri, ClientREST};

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
