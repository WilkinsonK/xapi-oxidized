use async_trait::async_trait;

use crate::client::{Xnat, ClientCore, ClientREST};
use crate::models::{Experiment, Project, Resource, Scan, Subject};
use crate::uri::data::resources::ResourcesUriBuilder;
use crate::uri::data::{
    ExperimentUri,
    ProjectUriLegacy,
    SubjectUriLegacy,
};
use crate::version::Version;
use crate::UriBuilder;
use super::crud::{CrudError, Create};

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
impl<V> Create<Project> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ProjectUriLegacy,
{
    async fn create_once(&self, model: Project) -> anyhow::Result<Project> {
        let mut model_clone = model.clone();
        let project = acquire_identifier!(model_clone.id, "project id")?;

        self.put(&self.version().project_data().with_id(project))
            .await?
            .json(&model_clone)
            .send()
            .await?;
        Ok(model)
    }
}

#[async_trait(?Send)]
impl<V> Create<Subject> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ProjectUriLegacy + SubjectUriLegacy,
{
    async fn create_once(&self, model: Subject) -> anyhow::Result<Subject> {
        let mut model_clone = model.clone();
        let project = acquire_identifier!(model_clone.project, "project id")?;
        let subject = acquire_identifier!(model_clone.id, "subject id")?;

        let uri = self
            .version()
            .project_data()
            .with_id(project)
            .subjects()
            .with_subject(subject);
        self
            .put(&uri)
            .await?
            .json(&model_clone)
            .send()
            .await?;
        Ok(model)
    }
}

#[async_trait(?Send)]
impl<V> Create<Experiment> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ExperimentUri + ProjectUriLegacy + SubjectUriLegacy,
{
    async fn create_once(&self, model: Experiment) -> anyhow::Result<Experiment> {
        let mut model_clone = model.clone();
        let project = acquire_identifier!(model_clone.project, "project id")?;
        let subject = acquire_identifier!(
            model_clone
                .subject_id
                .as_ref()
                .or(model_clone.subject_label.as_ref()),
            "subject id")?;
        let session = acquire_identifier!(
            model_clone
                .id
                .as_ref()
                .or(model_clone.label.as_ref()),
            "experiment id")?;

        let uri = self
            .version()
            .project_data()
            .with_id(project)
            .subjects()
            .with_subject(subject)
            .experiments()
            .with_experiment(session);
        self.put(&uri)
            .await?
            .json(&model_clone)
            .send()
            .await?;
        Ok(model)
    }
}

#[async_trait(?Send)]
impl<V> Create<Scan> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ExperimentUri + ProjectUriLegacy + SubjectUriLegacy,
{
    async fn create_once(&self, model: Scan) -> anyhow::Result<Scan> {
        let mut model_clone = model.clone();
        let project = acquire_identifier!(model_clone.project, "project id")?;
        let subject = acquire_identifier!(model_clone.subject, "subject id")?;
        let session = acquire_identifier!(model_clone.experiment, "experiment id")?;
        let scan = acquire_identifier!(model_clone.id,"scan id")?;

        let uri = self
            .version()
            .project_data()
            .with_id(project)
            .subjects()
            .with_subject(subject)
            .experiments()
            .with_experiment(session);
        self.put(&uri.scans().with_scan(scan))
            .await?
            .json(&model_clone)
            .send()
            .await?;
        Ok(model)
    }
}

#[async_trait(?Send)]
impl<V> Create<Resource> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ExperimentUri + ProjectUriLegacy + SubjectUriLegacy,
{
    async fn create_once(&self, model: Resource) -> anyhow::Result<Resource> {
        let mut model_clone = model.clone();
        model_clone.project.take();
        model_clone.subject.take();
        model_clone.experiment.take();
        model_clone.scan.take();
        model_clone.id.take();
        model_clone.collection.take();
        model_clone.name.take();

        let uri = match &model {
            Resource {
                project: Some(pjt),
                subject: Some(sbj),
                experiment: Some(exp),
                scan: Some(scn),
                ..
            } => {
                self.version()
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
                self.version()
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
                self.version()
                    .project_data()
                    .with_id(pjt)
                    .subjects()
                    .with_subject(sbj)
                    .build()
            },
            Resource {
                project: Some(pjt),
                ..
            } => self.version().project_data().with_id(pjt).build(),
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
            _ => uri
        };

        self.put(&uri)
            .await?
            .json(&model_clone)
            .send()
            .await?;
        Ok(model)
    }
}
