use async_trait::async_trait;

use crate::client::{Xnat, ClientCore, ClientREST};
use crate::models::common::ModelField;
use crate::uri::data::{
    ExperimentUri,
    ProjectUriLegacy,
    SubjectUriLegacy,
};
use crate::models::{
    Assessor, Experiment, FormatSpecifier, Plugin, Project, Resource, Scan, Subject
};
use crate::version::Version;
use crate::PluginUri;
use super::crud::Retrieve;
use super::CrudError;

macro_rules! retrieve_rst_vec {
    ($client:ident, $uri:ident, $model:ident) => {
        $client
            .get_any_result_from(&$uri, &$model)
            .await?
            .results()
            .to_vec()
    };
}

macro_rules! retrieve_its_vec {
    ($client:ident, $uri:ident, $model:ident) => {
        $client
            .get_any_items_from(&$uri, &$model)
            .await?
            .iter()
            .map(|i| i.unwrap())
            .collect::<Vec<_>>()
    }
}

macro_rules! retrieve_vec {
    ($client:ident, $uri:ident, $model:ident, $as_items:expr) => {
        if $as_items {
            retrieve_its_vec!($client, $uri, $model)
        } else {
            retrieve_rst_vec!($client, $uri, $model)
        }
    }
}

macro_rules! set_resources {
    ($uri:ident, $model:ident) => {
        {
            let $uri = $uri.resources();
            let $uri = if let Some(c) = &$model.collection {
                $uri.with_resource(c)
            } else { $uri };
            let $uri = if let Some(f) = &$model.name {
                $uri.with_file(f)
            } else { $uri };
            $uri
        }
    };
}

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
                retrieve_its_vec!(self, uri, model_clone)
            },
            None => retrieve_rst_vec!(self, uri, model_clone)
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
            retrieve_rst_vec!(self, uri, model_clone)
        } else if get_as_item {
            retrieve_its_vec!(self, uri, model_clone)
        } else {
            retrieve_rst_vec!(self, uri, model_clone)
        };
        Ok(data)
    }
}

#[async_trait(?Send)]
impl<V> Retrieve<Experiment> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ProjectUriLegacy + SubjectUriLegacy + ExperimentUri,
{
    async fn get_any_from(&self, model: &Experiment) -> anyhow::Result<Vec<Experiment>> {
        let mut model_clone = model.clone();

        // Filter over model values that are only
        // useful as URI params.
        let experiment = &model_clone
            .id
            .clone()
            .or(model_clone.label);

        let subject = &model_clone
            .subject_id
            .clone()
            .or(model_clone.subject_label);

        let project = &model_clone
            .project
            .clone()
            .or(model_clone.subject_project);

        // Clear out identifiers to avoid
        // polluting query params.
        model_clone.project         = None;
        model_clone.subject_project = None;
        model_clone.id              = None;
        model_clone.label           = None;
        model_clone.subject_id      = None;
        model_clone.subject_label   = None;
        // Set returning format to JSON.
        model_clone.format = Some(FormatSpecifier::Json);

        // When specifying the experiment, we are
        // expecting an item response.
        let data = if experiment.is_some() {
            let experiment = experiment.clone().unwrap();
            match [subject, project] {
                [Some(s), Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .subjects()
                        .with_subject(s)
                        .experiments()
                        .with_experiment(experiment);
                    retrieve_its_vec!(self, uri, model_clone)
                },
                [None, Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .experiments()
                        .with_experiment(experiment);
                    retrieve_its_vec!(self, uri, model_clone)
                },
                [Some(s), None] => {
                    let uri = self
                        .version()
                        .subject_data()
                        .with_subject(s)
                        .experiments()
                        .with_experiment(experiment);
                    retrieve_its_vec!(self, uri, model_clone)
                },
                [None, None] => {
                    let uri = self
                        .version()
                        .experiment_data()
                        .with_experiment(experiment);
                    retrieve_its_vec!(self, uri, model_clone)
                },
            }
        } else {
            match [subject, project] {
                [Some(s), Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .subjects()
                        .with_subject(s)
                        .experiments();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
                [None, Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .experiments();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
                [Some(s), None] => {
                    let uri = self
                        .version()
                        .subject_data()
                        .with_subject(s)
                        .experiments();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
                [None, None] => {
                    let uri = self
                        .version()
                        .experiment_data();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
            }
        };
        Ok(data)
    }
}

#[async_trait(?Send)]
impl<V> Retrieve<Assessor> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ProjectUriLegacy + SubjectUriLegacy + ExperimentUri,
{
    async fn get_any_from(&self, model: &Assessor) -> anyhow::Result<Vec<Assessor>> {
        let model_experiment = Experiment::from(model.clone());
        let mut model_clone = model.clone();

        // Filter over model values that are only
        // useful as URI params.
        // Filter over model values that are only
        // useful as URI params.
        let assessor = &model_clone.id.clone();

        let experiment = &model_experiment
            .id
            .clone()
            .or(model_experiment.label);

        let subject = &model_experiment
            .subject_id
            .clone()
            .or(model_experiment.subject_label);

        let project = &model_experiment
            .project
            .clone()
            .or(model_experiment.subject_project);

        let experiment = if let Some(e) = experiment {
            e
        } else {
            return Err(CrudError::IdentifierRequired("experiment id or label".into()).into())
        };

        // Clear out identifiers to avoid
        // polluting query params.
        model_clone.id = None;

        // Set returning format to JSON.
        model_clone.format = Some(FormatSpecifier::Json);

        // When specifying the experiment, we are
        // expecting an item response.
        let data = if assessor.is_some() {
            let assessor = assessor.as_ref().unwrap();
            match [subject, project] {
                [Some(s), Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .subjects()
                        .with_subject(s)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.assessors().with_assessor(assessor);
                    retrieve_its_vec!(self, uri, model_clone)
                },
                [None, Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.assessors().with_assessor(assessor);
                    retrieve_its_vec!(self, uri, model_clone)
                },
                [Some(s), None] => {
                    let uri = self
                        .version()
                        .subject_data()
                        .with_subject(s)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.assessors().with_assessor(assessor);
                    retrieve_its_vec!(self, uri, model_clone)
                },
                [None, None] => {
                    let uri = self
                        .version()
                        .experiment_data()
                        .with_experiment(experiment);
                    let uri = uri.assessors().with_assessor(assessor);
                    retrieve_its_vec!(self, uri, model_clone)
                },
            }
        } else {
            match [subject, project] {
                [Some(s), Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .subjects()
                        .with_subject(s)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.assessors();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
                [None, Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.assessors();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
                [Some(s), None] => {
                    let uri = self
                        .version()
                        .subject_data()
                        .with_subject(s)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.assessors();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
                [None, None] => {
                    let uri = self
                        .version()
                        .experiment_data()
                        .with_experiment(experiment);
                    let uri = uri.assessors();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
            }
        };
        Ok(data)
    }
}

#[async_trait(?Send)]
impl<V> Retrieve<Plugin> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + PluginUri,
{
    async fn get_any_from(&self, model: &Plugin) -> anyhow::Result<Vec<Plugin>> {
        let uri = self.version().plugins();
        let mut model_clone = model.clone();
        model_clone.name.take();

        Ok(if let Some(n) = &model.name {
            let uri = uri.with_plugin(n.property());
            retrieve_its_vec!(self, uri, model_clone)
        } else {
            retrieve_rst_vec!(self, uri, model_clone)
        })
    }
}

#[async_trait(?Send)]
impl<V> Retrieve<Resource> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ProjectUriLegacy + SubjectUriLegacy + ExperimentUri,
{
    async fn get_any_from(&self, model: &Resource) -> anyhow::Result<Vec<Resource>> {
        let mut model_clone = model.clone();
        model_clone.project.take();
        model_clone.subject.take();
        model_clone.experiment.take();
        model_clone.scan.take();
        model_clone.collection.take();
        model_clone.name.take();
        model_clone.format = Some("json".into());

        let take_as_items = model
            .collection
            .as_ref()
            .and(model.name.as_ref())
            .is_some();

        let data = match model {
            Resource {
                project: Some(pjt),
                subject: Some(sbj),
                experiment: Some(exp),
                scan: Some(scn),
                ..
            } => {
                let uri = self
                    .version()
                    .project_data()
                    .with_id(pjt)
                    .subjects()
                    .with_subject(sbj)
                    .experiments()
                    .with_experiment(exp);
                let uri = uri.scans().with_scan(scn);
                let uri = set_resources!(uri, model);
                retrieve_vec!(self, uri, model_clone, take_as_items)
            },
            Resource {
                project: Some(pjt),
                subject: Some(sbj),
                experiment: Some(exp),
                ..
            } => {
                let uri = self
                    .version()
                    .project_data()
                    .with_id(pjt)
                    .subjects()
                    .with_subject(sbj)
                    .experiments()
                    .with_experiment(exp);
                let uri = set_resources!(uri, model);
                retrieve_vec!(self, uri, model_clone, take_as_items)
            },
            Resource {
                project: Some(pjt),
                subject: Some(sbj),
                ..
            } => {
                let uri = self
                    .version()
                    .project_data()
                    .with_id(pjt)
                    .subjects()
                    .with_subject(sbj);
                let uri = set_resources!(uri, model);
                retrieve_vec!(self, uri, model_clone, take_as_items)
            },
            Resource {
                project: Some(pjt),
                experiment: Some(exp),
                scan: Some(scn),
                ..
            } => {
                let uri = self
                    .version()
                    .project_data()
                    .with_id(pjt)
                    .experiments()
                    .with_experiment(exp);
                let uri = uri.scans().with_scan(scn);
                let uri = set_resources!(uri, model);
                retrieve_vec!(self, uri, model_clone, take_as_items)
            },
            Resource { project: Some(pjt), experiment: Some(exp), .. } => {
                let uri = self
                    .version()
                    .project_data()
                    .with_id(pjt)
                    .experiments()
                    .with_experiment(exp);
                let uri = set_resources!(uri, model);
                retrieve_vec!(self, uri, model_clone, take_as_items)
            },
            Resource { project: Some(pjt), .. } => {
                let uri = self.version().project_data().with_id(pjt);
                let uri = set_resources!(uri, model);
                retrieve_vec!(self, uri, model_clone, take_as_items)
            },
            Resource { subject: Some(sbj), .. } => {
                let uri = self.version().subject_data().with_subject(sbj);
                let uri = set_resources!(uri, model);
                retrieve_vec!(self, uri, model_clone, take_as_items)
            },
            Resource { experiment: Some(exp), scan: Some(scn), .. } => {
                let uri = self.version().experiment_data().with_experiment(exp);
                let uri = uri.scans().with_scan(scn);
                let uri = set_resources!(uri, model);
                retrieve_vec!(self, uri, model_clone, take_as_items)
            },
            Resource { experiment: Some(exp), .. } => {
                let uri = self.version().experiment_data().with_experiment(exp);
                let uri = set_resources!(uri, model);
                retrieve_vec!(self, uri, model_clone, take_as_items)
            }
            _ => return Err(CrudError::IdentifierRequired("any identifier".into()).into())
        };
        Ok(data)
    }
}

#[async_trait(?Send)]
impl<V> Retrieve<Scan> for Xnat<V>
where
    Self: ClientCore<Version = V> + ClientREST,
    V: Version + ProjectUriLegacy + SubjectUriLegacy + ExperimentUri,
{
    async fn get_any_from(&self, model: &Scan) -> anyhow::Result<Vec<Scan>> {
        let mut model_clone = model.clone();

        // Filter over model values that are only
        // useful as URI params.
        let experiment = if let Some(e) = &model_clone.experiment {
            e
        } else {
            return Err(CrudError::IdentifierRequired("experiment id or label".into()).into())
        };
        let scan = &model_clone.id.clone();
        let subject = &model_clone.subject.clone();
        let project = &model_clone.project.clone();

        // Clear out identifiers to avoid
        // polluting query params.
        model_clone.id = None;

        // Set returning format to JSON.
        model_clone.format = Some(FormatSpecifier::Json);

        // When specifying the experiment, we are
        // expecting an item response.
        let data = if scan.is_some() {
            let scan = scan.as_ref().unwrap();
            match [subject, project] {
                [Some(s), Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .subjects()
                        .with_subject(s)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.scans().with_scan(scan);
                    retrieve_its_vec!(self, uri, model_clone)
                },
                [None, Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.scans().with_scan(scan);
                    retrieve_its_vec!(self, uri, model_clone)
                },
                [Some(s), None] => {
                    let uri = self
                        .version()
                        .subject_data()
                        .with_subject(s)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.scans().with_scan(scan);
                    retrieve_its_vec!(self, uri, model_clone)
                },
                [None, None] => {
                    let uri = self
                        .version()
                        .experiment_data()
                        .with_experiment(experiment);
                    let uri = uri.scans().with_scan(scan);
                    retrieve_its_vec!(self, uri, model_clone)
                },
            }
        } else {
            match [subject, project] {
                [Some(s), Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .subjects()
                        .with_subject(s)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.scans();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
                [None, Some(p)] => {
                    let uri = self
                        .version()
                        .project_data()
                        .with_id(p)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.scans();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
                [Some(s), None] => {
                    let uri = self
                        .version()
                        .subject_data()
                        .with_subject(s)
                        .experiments()
                        .with_experiment(experiment);
                    let uri = uri.scans();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
                [None, None] => {
                    let uri = self
                        .version()
                        .experiment_data()
                        .with_experiment(experiment);
                    let uri = uri.scans();
                    retrieve_rst_vec!(self, uri, model_clone)
                },
            }
        };
        Ok(data)
    }
}
