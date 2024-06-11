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
    Items,
    Project,
    ResultSet,
    Subject,
};
use crate::version::Version;
use super::crud::{Retrieve, try_retrieve};

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
        if model_clone.id.is_some() {
            uri = uri.with_id(model.id.as_ref().unwrap());
            model_clone.id = None;
        }

        let res = self
            .get(&uri)
            .await?
            .query(&model_clone)
            .send()
            .await?;

        try_retrieve(res, |r| async {
            match &model.id {
                Some(_) => r
                    .json::<Items<Project>>()
                    .await
                    .expect("item set parsed")
                    .iter()
                    .map(|i| i.unwrap())
                    .collect::<Vec<_>>(),
                None => r
                    .json::<ResultSet<Project>>()
                    .await
                    .expect("result set parsed")
                    .results()
                    .to_vec(),
            }
        }).await
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

        model_clone.format = Some(FormatSpecifier::Json);
        if model_clone.id.is_some() {
            uri = uri.with_subject(model.id.as_ref().unwrap().to_string());
            model_clone.id = None;
        }

        let res = self
            .get(&uri)
            .await?
            .query(&model_clone)
            .send()
            .await?;

        try_retrieve(res, |r| async {
            match model.id {
                Some(_) => r
                    .json::<Items<Subject>>()
                    .await
                    .expect("item set parsed")
                    .iter()
                    .map(|i| i.unwrap())
                    .collect::<Vec<_>>(),
                None => r
                    .json::<ResultSet<Subject>>()
                    .await
                    .expect("result set parsed")
                    .results()
                    .to_vec()
            }
        }).await
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
        if model_clone.id.is_some() {
            uri = uri.with_experiment(model.id.as_ref().unwrap());
            model_clone.id = None;
        }

        let res = self
            .get(&uri)
            .await?
            .query(&model_clone)
            .send()
            .await?;

        try_retrieve(res, |r| async {
            match model.id {
                Some(_) => r
                    .json::<Items<Experiment>>()
                    .await
                    .expect("item set parsed")
                    .iter()
                    .map(|i| i.unwrap())
                    .collect::<Vec<_>>(),
                None => r
                    .json::<ResultSet<Experiment>>()
                    .await
                    .expect("result set parsed")
                    .results()
                    .to_vec()
            }
        }).await
    }
}
