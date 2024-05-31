mod common;

use oxinat::*;
use oxinat_core::{
    ClientBuilderAttrs,
    ClientBuilderCore,
    ClientBuilderToken,
    ClientCore,
    Version,
    Xnat,
    XnatBuilder
};

fn setup_builder<V: Version + Clone>(version: V) -> XnatBuilder<V> {
    common::init();
    Xnat::configure(&common::env_hostname())
        .with_version(version)
        .with_password(&common::env_password())
        .with_username(&common::env_username())
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
    assert_ne!(client.as_ref().unwrap().get_session_id(), "");
    drop(client);

    let client = builder.acquire().await;
    assert!(client.is_ok(), "must be able to build client without any errors")
}
