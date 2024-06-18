use async_trait::async_trait;

use crate::uri::data::resources::ResourcesUriBuilder;
use crate::UriBuilder;
use crate::client::{Xnat, ClientCore, ClientREST};
use crate::models::{Experiment, Project, Resource, Scan, Subject};
use crate::uri::data::{ExperimentUri, ProjectUriLegacy, SubjectUriLegacy};
use crate::version::Version;
use super::crud::{CrudError, Delete};

/// Takes the `Option` value for the specified
/// attribute, returning a `Result`. Otherwise
/// fails and returns a
/// `CrudError::IdentifierRequired`.
macro_rules! acquire_identifier {
    ($attr:expr, $error:literal) => {
        $attr.take().ok_or(CrudError::IdentifierRequired($error.to_string()))
    };
}

#[async_trait(?Send)]
impl<V> Delete<Project> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ProjectUriLegacy,
{
    async fn delete_once(&self, model: Project) -> anyhow::Result<Project> {
        let mut model_clone = model.clone();
        let project = acquire_identifier!(model_clone.id, "project id")?;

        self.delete(&self.version().project_data().with_id(project))
            .await?
            .send()
            .await?;
        Ok(model)
    }
}

#[async_trait(?Send)]
impl<V> Delete<Subject> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ProjectUriLegacy + SubjectUriLegacy,
{
    async fn delete_once(&self, model: Subject) -> anyhow::Result<Subject> {
        let mut model_clone = model.clone();
        let project = acquire_identifier!(model_clone.project, "project id")?;
        let subject = acquire_identifier!(
            model_clone
                .label
                .as_ref()
                .or(model_clone.id.as_ref()),
            "subject id")?;

        let uri = self.version()
            .project_data()
            .with_id(project)
            .subjects()
            .with_subject(subject);
        self.delete(&uri).await?.send().await?;
        Ok(model)
    }
}

#[async_trait(?Send)]
impl<V> Delete<Experiment> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ExperimentUri + ProjectUriLegacy + SubjectUriLegacy,
{
    async fn delete_once(&self, model: Experiment) -> anyhow::Result<Experiment> {
        let model_clone = model.clone();
        let project = acquire_identifier!(
            model_clone
                .project
                .as_ref()
                .or(model_clone.subject_project.as_ref()),
            "project id")?;
        let subject = acquire_identifier!(
            model_clone
                .subject_label
                .as_ref()
                .or(model_clone.subject_id.as_ref()),
            "subject id")?;
        let session = acquire_identifier!(
            model_clone
                .id
                .as_ref()
                .or(model_clone.label.as_ref()),
            "experiment id")?;

        let uri = self.version()
            .project_data()
            .with_id(project)
            .subjects()
            .with_subject(subject)
            .experiments()
            .with_experiment(session);
        self.delete(&uri).await?.send().await?;
        Ok(model)
    }
}

#[async_trait(?Send)]
impl<V> Delete<Scan> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ExperimentUri + ProjectUriLegacy + SubjectUriLegacy,
{
    async fn delete_once(&self, model: Scan) -> anyhow::Result<Scan> {
        let mut model_clone = model.clone();

        let project = acquire_identifier!(
            model_clone.project,
            "project id")?;
        let subject = acquire_identifier!(
            model_clone.subject,
            "subject id")?;
        let session = acquire_identifier!(
            model_clone.experiment,
            "experiment id")?;
        let scan = acquire_identifier!(model_clone.id, "scan id")?;

        let uri = self.version()
            .project_data()
            .with_id(project)
            .subjects()
            .with_subject(subject)
            .experiments()
            .with_experiment(session);
        self.delete(&uri.scans().with_scan(scan)).await?.send().await?;
        Ok(model)
    }
}

#[async_trait(?Send)]
impl<V> Delete<Resource> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ExperimentUri + ProjectUriLegacy + SubjectUriLegacy,
{
    async fn delete_once(&self, model: Resource) -> anyhow::Result<Resource> {
        let uri = match &model {
            Resource {
                project: Some(pjt),
                subject: Some(sbj),
                experiment: Some(exp),
                scan: Some(scn),
                ..
            } => {
                self
                    .version()
                    .project_data()
                    .with_id(pjt)
                    .subjects()
                    .with_subject(sbj)
                    .experiments()
                    .with_experiment(exp)
                    .scans()
                    .with_scan(scn)
                    .build()
            },
            Resource {
                project: Some(pjt),
                subject: Some(sbj),
                experiment: Some(exp),
                ..
            } => {
                self
                    .version()
                    .project_data()
                    .with_id(pjt)
                    .subjects()
                    .with_subject(sbj)
                    .experiments()
                    .with_experiment(exp)
                    .build()
            },
            Resource {
                project: Some(pjt),
                subject: Some(sbj),
                ..
            } => {
                self
                    .version()
                    .project_data()
                    .with_id(pjt)
                    .subjects()
                    .with_subject(sbj)
                    .build()
            },
            Resource {
                project: Some(pjt),
                ..
            } => {
                self
                    .version()
                    .project_data()
                    .with_id(pjt)
                    .build()
            },
            _ => return Err(CrudError::IdentifierRequired("any identifiers".into()).into())
        }?;
        let uri = ResourcesUriBuilder::default().with_parent(&uri);
        let uri = match &model {
            Resource { collection: Some(c), name: Some(n), .. } => {
                uri.with_resource(c).with_file(n)
            },
            Resource { collection: Some(c), .. } => {
                uri.with_resource(c)
            },
            _ => uri,
        };

        self.delete(&uri).await?.send().await?;
        Ok(model)
    }
}
