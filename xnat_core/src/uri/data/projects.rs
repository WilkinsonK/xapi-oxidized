use std::{fmt::Debug, path::PathBuf, sync::Arc};

use oxinat_derive::uri_builder_alias;

use crate::{UriBuildError, UriBuilder, Version};

use super::{experiments::ExperimentUriLegacyBuilder, resources::ResourcesUriBuilder, subjects::SubjectUriLegacyBuilder};

uri_builder_alias!(ProjectDataUriBuilder);
ImplProjectDataUriBuilder! {
    (String),
}

/// URI endpoint paths for project data
/// management.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}")] // Passthrough argument.
pub struct ProjectUriBuilder<Parent>
where
    Parent: ProjectDataUriBuilder,
{
    #[parent]
    parent: Option<Arc<Parent>>
}

/// Represents URI endpoint paths available for
/// project investigator management.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/investigators")]
#[match_path(path = "{parent}/investigators/investigator_id")]
pub struct InvestigatorsUriBuilder<'a> {
    #[param]
    investigator_id: Option<String>,
    #[parent]
    parent: Option<&'a ProjectUriBuilder<String>>
}

impl ProjectUriBuilder<String> {
    /// Continue the builder into a
    /// `InvestigatorsUriBuilder`.
    pub fn investigators(&self) -> InvestigatorsUriBuilder {
        InvestigatorsUriBuilder::from_parent(self)
    }
}

macro_rules! parent_has_id {
    () => {
        |this: &Self| this.parent.is_some_and(|p| p.id.is_some())
    };
}

/// Legacy URI endpoint paths for project data
/// management.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/projects")]
#[match_path(path = "{parent}/projects/{id}")]
pub struct ProjectUriLegacyBuilder<Parent>
where
    Parent: ProjectDataUriBuilder,
{
    subject:    Option<String>,
    experiment: Option<String>,

    #[param]
    id: Option<String>,
    #[parent]
    parent: Option<Arc<Parent>>
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ProjectAttributeType {
    Accessibility,
    CurrentArc,
    Prearchive,
    Quarantine,
    ScanTypes,
    #[default]
    None
}

macro_rules! has_project_attr {
    ($type:ident) => {
        (|this: &Self|
            this.attribute_type == ProjectAttributeType::$type
            && this.parent.is_some_and(|p| p.id.is_some())
        )
    };
}

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/accessibility", requires = "has_project_attr!(Accessibility)")]
#[match_path(path = "{parent}/accessibility/{status}", requires = "has_project_attr!(Accessibility)")]
#[match_path(path = "{parent}/current_arc", requires = "has_project_attr!(CurrentArc)")]
#[match_path(path = "{parent}/prearchive_code", requires = "has_project_attr!(Prearchive)")]
#[match_path(path = "{parent}/prearchive_code/{code}", requires = "has_project_attr!(Prearchive)")]
#[match_path(path = "{parent}/quarantine_code", requires = "has_project_attr!(Quarantine)")]
#[match_path(path = "{parent}/quarantine_code/{code}", requires = "has_project_attr!(Quarantine)")]
#[match_path(path = "{parent}/scan_types", requires = "has_project_attr!(ScanTypes)")]
pub struct AttributesUriBuilder<'a> {
    attribute_type: ProjectAttributeType,
    #[param]
    code: Option<String>,
    #[param]
    status: Option<String>,
    #[parent]
    parent: Option<&'a ProjectUriLegacyBuilder<String>>,
}

/// Represents the URI paths available for
/// managing users related to some XNAT project.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/users", requires = "parent_has_id!()")]
#[match_path(path = "{parent}/users/{group_name}/{username}", requires = "parent_has_id!()")]
pub struct UsersUriBuilder<'a> {
    #[param]
    group_name: Option<String>,
    #[param]
    username: Option<String>,
    #[parent]
    parent: Option<&'a ProjectUriLegacyBuilder<String>>,
}

/// Represents the URI paths available to manage
/// project configurations.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/config", requires = "parent_has_id!()")]
#[match_path(path = "{parent}/config/{tool_id}", requires = "parent_has_id!()")]
#[match_path(path = "{parent}/config/{tool_id}/file_path", requires = "parent_has_id!()")]
pub struct ConfigUriBuilder<'a> {
    #[param(map_from = "|pb: &PathBuf| pb.to_str().unwrap().to_string()")]
    file_path: Option<PathBuf>,
    #[param]
    tool_id: Option<String>,
    #[parent]
    parent: Option<&'a ProjectUriLegacyBuilder<String>>,
}

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/pipelines", requires = "parent_has_id!()")]
#[match_path(path = "{parent}/pipelines/{step}/experiments/{experiment}", requires = "parent_has_id!()")]
pub struct Pipelines<'a> {
    #[param]
    step: Option<String>,
    #[param]
    experiment: Option<String>,
    #[parent]
    parent: Option<&'a ProjectUriLegacyBuilder<String>>,
}

impl ProjectUriLegacyBuilder<String> {
    /// Continue the builder into a
    /// `AttributesUriBuilder`
    pub fn attributes(&self) -> AttributesUriBuilder {
        AttributesUriBuilder::from_parent(&Arc::new(self))
    }

    /// Produce the data/projects/{id}/pars URI
    /// path.
    pub fn build_pars(&self) -> crate::BuildResult {
        self
            .id
            .is_some()
            .then(|| self.build_join("pars"))
            .unwrap_or(Err(UriBuildError::Validation.into()))
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
    /// `SubjectUriLegacyBuilder`.
    pub fn subjects(&self) -> SubjectUriLegacyBuilder<Self> {
        let b = SubjectUriLegacyBuilder::from_parent(Arc::new(self.to_owned()));
        match self.subject.as_ref() {
            Some(sbj) => b.with_subject(sbj),
            _ => b
        }
    }

    /// Continue the builder into a
    /// `UsersUriBuilder`.
    pub fn users(&self) -> UsersUriBuilder {
        UsersUriBuilder::from_parent(&Arc::new(self))
    }
}

/// Represents the URI paths to access and modify
/// XNAT projects.
pub trait ProjectUri: Version {
    /// URI endpoints for manipulating project
    /// data.
    #[inline]
    fn projects(&self) -> ProjectUriBuilder<String> {
        ProjectUriBuilder::from_parent(self.root_uri().into())
    }
}

/// Represents the URI paths to access archive
/// paths for project data.
pub trait ProjectUriArchive: Version {
    /// URI paths for accessing project archive
    /// data.
    #[inline]
    fn project_archive(&self) -> ProjectUriBuilder<String> {
        ProjectUriBuilder::from_parent("archive".to_string().into())
    }
}

/// Represents the URI paths to access and modify
/// XNAT projects.
pub trait ProjectUriLegacy: Version {
    /// Legacy URI endpoints for manipulating
    /// project data.
    #[inline]
    fn project_data(&self) -> ProjectUriLegacyBuilder<String> {
        ProjectUriLegacyBuilder::from_parent(self.data_uri().into())
    }
}
