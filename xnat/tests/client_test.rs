mod common;

use oxinat::*;

#[test]
#[ignore = "must have a .env file or variables set in env"]
fn test_client_builder01() {
    common::init();

    let client = Xnat::configure(&common::env_hostname())
        .with_version(V2)
        .with_password(&common::env_password())
        .with_username(&common::env_username())
        .build();
    assert!(client.is_ok(), "must be able to build client without any errors")
}
