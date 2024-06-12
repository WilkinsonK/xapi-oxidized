mod common;

use oxinat::{
    models::{Experiment, Project, Subject},
    protocols::Retrieve, ClientToken
};
use oxinat_core::anyhow;

#[tokio::test]
#[ignore = "must have a .env file for variables set in env"]
async fn test_protocols_experiment_retreive01() {
    common::init();

    let mut client = common::request_client().await;

    let found: anyhow::Result<Vec<Experiment>> = client.get_all().await;
    assert!(found.is_ok(), "must be able to retrieve `Experiment`s from host: {found:?}");
    assert!(!found.unwrap().is_empty(), "results must have `some` items");

    client.release().await.unwrap();
}

#[tokio::test]
#[ignore = "must have a .env file for variables set in env"]
async fn test_protocols_project_retrieve01() {
    common::init();

    let mut client = common::request_client().await;

    let found: anyhow::Result<Vec<Project>> = client.get_all().await;
    assert!(found.is_ok(), "must be able to retrieve `Project`s from host: {found:?}");
    assert!(!found.unwrap().is_empty(), "results must have `some` items");

    client.release().await.unwrap();
}

#[tokio::test]
#[ignore = "must have a .env file for variables set in env"]
async fn test_protocols_project_retrieve02() {
    common::init();

    let mut client = common::request_client().await;
    let mut model = Project::default();
    model.id.clone_from(&common::env_project_id().into());

    let found = client.get_any_from(&model).await;
    assert!(found.is_ok(), "must be able to retrieve `Projects`s from host: {found:?}");
    assert!(!found.unwrap().is_empty(), "should contain at least one project");

    client.release().await.unwrap();
}

#[tokio::test]
#[ignore = "must hav a .env file for variables set in env"]
async fn test_protocols_subject_retrieve01() {
    common::init();

    let mut client = common::request_client().await;

    let found: anyhow::Result<Vec<Subject>> = client.get_all().await;
    assert!(found.is_ok(), "must be able to retrieve `Subject`s from host: {found:?}");
    assert!(!found.unwrap().is_empty(), "results must have `some` items");

    client.release().await.unwrap();
}

#[tokio::test]
#[ignore = "must have a .env file for variables set in env"]
async fn test_protocols_subject_retrieve02() {
    common::init();

    let mut client = common::request_client().await;
    let model = Subject::default();

    let found = client.get_any_from(&model).await;
    assert!(found.is_ok(), "must be able to retrieve `Subject`s from host: {found:?}");
    assert!(!found.unwrap().is_empty(), "should contain at least one subject");

    client.release().await.unwrap();
}

#[tokio::test]
#[ignore = "must have a .env file for variables set in env"]
async fn test_protocols_subject_retrieve03() {
    common::init();

    let mut client = common::request_client().await;
    let mut model = Subject::default();
    model.project.clone_from(&common::env_project_id().into());

    let found = client.get_any_from(&model).await;
    assert!(found.is_ok(), "must be able to retrieve `Subject`s from host: {found:?}");
    assert!(!found.unwrap().is_empty(), "should contain at least one subject");

    client.release().await.unwrap();
}
