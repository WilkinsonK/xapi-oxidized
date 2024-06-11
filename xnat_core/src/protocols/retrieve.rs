use async_trait::async_trait;

use crate::client::{Xnat, ClientCore, ClientREST};
use crate::uri::data::{
    ExperimentUri,
    ProjectUriLegacy,
    SubjectUriLegacy,
};
use crate::models::{
    Experiment,
    FormatSpecifier,
    Project,
    Subject,
};
use crate::version::Version;
use super::crud::Retrieve;

#[async_trait(?Send)]
impl<V> Retrieve<Project> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ProjectUriLegacy,
{
    async fn get_any_from(&self, model: &Project) -> anyhow::Result<Vec<Project>> {
        let mut uri = self.version().project_data();
        let mut model_clone = model.clone();

        model_clone.format = Some(FormatSpecifier::Json);
        Ok(match &model.id {
            Some(i) => {
                uri = uri.with_id(i);
                model_clone.id = None;
                self
                    .get_any_items_from(&uri, &model_clone)
                    .await?
                    .iter()
                    .map(|i| i.unwrap())
                    .collect::<Vec<_>>()
            },
            None => self
                .get_any_result_from(&uri, &model_clone)
                .await?
                .results()
                .to_vec()
        })
    }
}

#[async_trait(?Send)]
impl<V> Retrieve<Subject> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + SubjectUriLegacy,
{
    async fn get_any_from(&self, model: &Subject) -> anyhow::Result<Vec<Subject>> {
        let mut uri = self.version().subject_data();
        let mut model_clone = model.clone();
        // Ask the host to return a JSON response.
        model_clone.format = Some(FormatSpecifier::Json);
        // Identify the parameters to pass into
        // the URI.
        // We assume first that we will want to
        // get models as an `Items` collection.
        let mut get_as_item = true;
        match model_clone {
            Subject { label: Some(_), ..} => {
                uri = uri.with_subject(model_clone.label.take().unwrap());
                model_clone.id.clone_from(&None);
            },
            Subject { id: Some(_), ..} => {
                uri = uri.with_subject(model_clone.id.take().unwrap().to_string());
                model_clone.label.clone_from(&None);
            },
            _ => { get_as_item = false; }
        };

        let data = if let Subject { project: Some(p), .. } = &model_clone {
            let uri = uri.by_project(p);
            self
                .get_any_result_from(&uri, &model_clone)
                .await?
                .results()
                .to_vec()
        } else if get_as_item {
            self
                .get_any_items_from(&uri, &model_clone)
                .await?
                .iter()
                .map(|i| i.unwrap())
                .collect::<Vec<_>>()
        } else {
            self
                .get_any_result_from(&uri, &model_clone)
                .await?
                .results()
                .to_vec()
        };
        Ok(data)
    }
}

#[async_trait(?Send)]
impl<V> Retrieve<Experiment> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ExperimentUri,
{
    async fn get_any_from(&self, model: &Experiment) -> anyhow::Result<Vec<Experiment>> {
        let mut uri = self.version().experiment_data();
        let mut model_clone = model.clone();

        model_clone.format = Some(FormatSpecifier::Json);
        Ok(match &model.id {
            Some(_) => {
                uri = uri.with_experiment(model_clone.id.take().unwrap());
                self
                    .get_any_items_from(&uri, &model_clone)
                    .await?
                    .iter()
                    .map(|i| i.unwrap())
                    .collect::<Vec<_>>()
            },
            None => self
                .get_any_result_from(&uri, &model_clone)
                .await?
                .results()
                .to_vec()
        })
    }
}
