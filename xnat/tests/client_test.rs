mod common;

use oxinat::*;
use oxinat_core::{
    client::timeouts::Timeouts, ClientBuilderAttrs, ClientBuilderCore, ClientBuilderToken, ClientCore, Version, Xnat, XnatBuilder
};

fn setup_builder<V: Version + Clone>(version: V) -> XnatBuilder<V> {
    common::init();
    Xnat::configure(&common::env_hostname())
        .with_version(version)
        .with_password(&common::env_password())
        .with_username(&common::env_username())
        .with_timeouts(&Timeouts::default().with_connect_secs(300))
        .use_secure(true)
}

#[test]
#[ignore = "must have a .env file or variables set in env"]
fn test_client_builder01() {
    let client = setup_builder(V2).build();
    assert!(client.is_ok(), "must be able to build client without any errors")
}

#[tokio::test]
#[ignore = "must have a .env file or variables set in env"]
async fn test_client_builder02() {
    let builder = setup_builder(V2);

    let client = builder.acquire().await;
    assert!(client.is_ok(), "must be able to build client without any errors");

    let mut client = client.unwrap();
    assert_ne!(client.get_session_id(), "", "must be able to acquire a token");

    let release_result = client.release().await;
    assert!(release_result.is_ok(), "must be able to release the auth token: {release_result:?}");
}
