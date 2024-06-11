mod common;

use oxinat::{
    models::Project,
    protocols::Retrieve
};
use oxinat_core::anyhow;

#[tokio::test]
#[ignore = "must have a .env file for variables set in env"]
async fn test_protocols_project_retrieve01() {
    common::init();

    let client = common::request_client().await;

    let found: anyhow::Result<Vec<Project>> = client.get_all().await;
    assert!(found.is_ok(), "must be able to retrieve `Project`s from host: {found:?}");
    assert!(!found.unwrap().is_empty(), "results must have `some` items")
}

#[tokio::test]
#[ignore = "must have a .env file for variables set in env"]
async fn test_protocols_project_retrieve02() {
    common::init();

    let client = common::request_client().await;
    let model = Project {
        description: None,
        id: Some(common::env_project_id()),
        name: None,
        pi_firstname: None,
        pi_lastname: None,
        secondary_id: None,
        uri: None,

        format: None,
        subjects: None,
    };

    let found = client.get_any_from(&model).await;
    assert!(found.is_ok(), "must be able to retrieve `Projects`s from host: {found:?}");
    assert!(!found.unwrap().is_empty(), "should contain at least one project");
}
