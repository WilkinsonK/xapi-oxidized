use std::{fmt::Debug, rc::Rc};

use oxinat_derive::uri_builder_alias;

use crate::{UriBuilder, Version};

use super::{
    projects::{ProjectDataUriBuilder, ProjectUriLegacyBuilder},
    resources::ResourcesUriBuilder,
    shared::SharedProjectUriBuilder,
    subjects::{SubjectDataUriBuilder, SubjectUriLegacyBuilder}
};

uri_builder_alias!(ExperimentDataUriBuilder);
ImplExperimentDataUriBuilder! {
    (String),
}
ImplExperimentDataUriBuilder! {
    (ExperimentUriLegacyBuilder<Parent>, Parent),
}

impl<Parent> ExperimentDataUriBuilder for ProjectUriLegacyBuilder<Parent>
where
    Parent: ExperimentDataUriBuilder + ProjectDataUriBuilder,
{}

impl<Parent> ExperimentDataUriBuilder for SubjectUriLegacyBuilder<Parent>
where
    Parent: ExperimentDataUriBuilder + SubjectDataUriBuilder,
{}

/// Represents the URI endpoints available to
/// manage XNAT experiment sessions.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/experiments")]
#[match_path(path = "{parent}/experiments/{experiment}")]
pub struct ExperimentUriLegacyBuilder<Parent>
where
    Parent: ExperimentDataUriBuilder,
{
    #[param]
    experiment: Option<String>,
    #[parent]
    parent: Option<Rc<Parent>>
}

macro_rules! parent_has_experiment {
    () => {
        |this: &Self| this.experiment.is_some()
    };
}

/// Represents the URI endpoints available to
/// manage XNAT experiment scans.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/scans", requires = "parent_has_experiment!()")]
#[match_path(path = "{parent}/scans/{scan}", requires = "parent_has_experiment!()"  )]
pub struct ExperimentScanUriBuilder<'a, Parent>
where
    Parent: ExperimentDataUriBuilder,
{
    experiment: Option<String>,

    #[param]
    scan: Option<u64>,
    #[parent]
    parent: Option<&'a Parent>
}

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/assessors", requires = "parent_has_experiment!()")]
#[match_path(path = "{parent}/assessors/{assessor}", requires = "parent_has_experiment!()")]
pub struct AssessorUriBuilder<'a, Parent>
where
    Parent: ExperimentDataUriBuilder,
{
    experiment: Option<String>,

    #[param]
    assessor: Option<String>,
    #[parent]
    parent: Option<&'a Parent>
}

impl<Parent> ExperimentUriLegacyBuilder<Parent>
where
    Parent: ExperimentDataUriBuilder + Default,
{
    /// Continue the builder into a
    /// `AssessorUriBuilder`.
    pub fn assessors(&self) -> AssessorUriBuilder<'_, Self> {
        let mut b = AssessorUriBuilder::from_parent(self);
        b.experiment.clone_from(&self.experiment);
        b
    }

    /// Continue the builder into a
    /// `ResourceUriBuilder`.
    pub fn resources(&self) -> ResourcesUriBuilder<'_, Self> {
        ResourcesUriBuilder::from_parent(&Rc::new(self))
    }

    /// Continue the builder into a
    /// `ExperimentScanUriBuilder`.
    pub fn scans(&self) -> ExperimentScanUriBuilder<'_, Self> {
        let mut b = ExperimentScanUriBuilder::from_parent(self);
        b.experiment.clone_from(&self.experiment);
        b
    }

    /// Continue the builder into a
    /// `SharedProjectUriBuilder`.
    pub fn shared(&self) -> SharedProjectUriBuilder<'_, Self> {
        SharedProjectUriBuilder::from_parent(&Rc::new(self))
    }

    /// Produce the quarantine status URI path.
    pub fn quarantine_status(&self) -> anyhow::Result<String> {
        self.build_join("status")
    }
}

impl ExperimentUriLegacyBuilder<String> {
    /// Reconstrucct this builder to allow search
    /// by project.
    pub fn by_project(&self, project: &str) -> ExperimentUriLegacyBuilder<ProjectUriLegacyBuilder<String>> {
        let parent = self.parent.as_ref().unwrap().clone();
        let b = ProjectUriLegacyBuilder::from_parent(parent)
            .with_id(&project)
            .experiments();
        match self.experiment.as_ref() {
            Some(exp) => b.with_experiment(&exp),
            _ => b
        }
    }

    /// Reconstruct this builder to allow search
    /// by subject.
    pub fn by_subject(&self, subject: &str) -> ExperimentUriLegacyBuilder<SubjectUriLegacyBuilder<String>> {
        let parent = self.parent.as_ref().unwrap().clone();
        let b = SubjectUriLegacyBuilder::from_parent(parent)
            .with_subject(&subject)
            .experiments();
        match self.experiment.as_ref() {
            Some(exp) => b.with_experiment(&exp),
            _ => b
        }
    }
}

/// Represents the URI endpoints available for
/// XNAT experiment management.
pub trait ExperimentUri: Version {
    #[inline]
    fn experiment_data(&self) -> ExperimentUriLegacyBuilder<String> {
        ExperimentUriLegacyBuilder::from_parent(self.data_uri().into())
    }
}
