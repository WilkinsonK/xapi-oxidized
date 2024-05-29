use std::{fmt::Debug, sync::Arc};

use oxinat_derive::uri_builder_alias;

use crate::{UriBuilder, Version};

use super::{
    experiments::ExperimentUriLegacyBuilder,
    projects::ProjectUriLegacyBuilder,
    resources::ResourcesUriBuilder,
    shared::SharedProjectUriBuilder,
};

uri_builder_alias!(SubjectDataUriBuilder);
ImplSubjectDataUriBuilder! {
    (String),
    (ProjectUriLegacyBuilder<String>)
}

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/subjects")]
#[match_path(path = "{parent}/subjects/{subject}")]
pub struct SubjectUriLegacyBuilder<Parent>
where
    Parent: SubjectDataUriBuilder,
{
    experiment: Option<String>,

    #[param]
    subject: Option<String>,
    #[parent]
    parent: Option<Arc<Parent>>
}

impl SubjectUriLegacyBuilder<String> {
    /// Reconstruct this builder to allow search
    /// by project.
    pub fn by_project(&self, project: &str) -> SubjectUriLegacyBuilder<ProjectUriLegacyBuilder<String>> {
        let parent = self.parent.as_ref().unwrap().clone();
        let mut b = ProjectUriLegacyBuilder::from_parent(parent)
            .with_id(project)
            .subjects();

        b = match self.subject.as_ref() {
            Some(sbj) => b.with_subject(sbj),
            _ => b
        };
        b = match self.experiment.as_ref() {
            Some(exp) => b.with_experiment(exp),
            _ => b
        };
        b
    }

    /// Continue the builder into a
    /// `ExperimentUriLegacyBuilder`.
    pub fn experiments(&self) -> ExperimentUriLegacyBuilder<Self> {
        let b = ExperimentUriLegacyBuilder::from_parent(Arc::new(self.to_owned()));
        match self.experiment.as_ref() {
            Some(exp) => b.with_experiment(exp),
            _ => b
        }
    }

    /// Continue the builder into a
    /// `ResourceUriBuilder`.
    pub fn resources(&self) -> ResourcesUriBuilder<'_, Self> {
        ResourcesUriBuilder::from_parent(&Arc::new(self))
    }

    /// Continue the builder into a
    /// `SharedProjectUriBuilder`.
    pub fn shared(&self) -> SharedProjectUriBuilder<'_, Self> {
        SharedProjectUriBuilder::from_parent(self)
    }
}

/// Represents the URI paths to access archive
/// paths for subject data.
pub trait SubjectUriArchive: Version {
    /// Represents the URI paths to access archive
    /// paths for subject data.
    #[inline]
    fn subject_archive(&self) -> SubjectUriLegacyBuilder<String> {
        SubjectUriLegacyBuilder::from_parent("archive".to_string().into())
    }
}

pub trait SubjectUriLegacy: Version {
    /// Legacy URI endpoints for manipulating
    /// subject data.
    #[inline]
    fn subject_data(&self) -> SubjectUriLegacyBuilder<String> {
        SubjectUriLegacyBuilder::from_parent(self.data_uri().into())
    }
}