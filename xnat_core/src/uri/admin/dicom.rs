use std::{fmt::Debug, sync::Arc};

use oxinat_derive::uri_builder_alias;

use crate::{UriBuilder, Version};

uri_builder_alias!(DicomAdminUriBuilder);
ImplDicomAdminUriBuilder! {
    (String),
}

macro_rules! id_is_none {
    () => {
        |this: &Self| this.parent.as_ref().is_some_and(|p| p.id.is_none())
    };
}

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}")] // URI passthough.
pub struct DicomUriBuilder<Parent>
where
    Parent: DicomAdminUriBuilder,
{
    #[parent]
    parent: Option<Arc<Parent>>
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ScpAction {
    Enabled,
    Start,
    Stop,
    #[default]
    None
}

macro_rules! is_scpaction {
    () => {
        (|this: &Self| this.action == ScpAction::default())
    };
    ($type:ident) => {
        (|this: &Self| this.action == ScpAction::$type)
    };
}

/// Represents paths available for managing DICOM
/// SCP servers.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/dicomscp/start", requires = "is_scpaction!(Start)")]
#[match_path(path = "{parent}/dicomscp/stop", requires = "is_scpaction!(Stop)")]
#[match_path(path = "{parent}/dicomscp/{id}/enabled", requires = "is_scpaction!(Enabled)")]
#[match_path(path = "{parent}/dicomscp/{id}/enabled/{enabled}", requires = "is_scpaction!(Enabled)")]
#[match_path(path = "{parent}/dicomscp/{id}")]
#[match_path(path = "{parent}/dicomscp")]
pub struct DicomScpUriBuilder<'a>
{
    action: ScpAction,
    #[param]
    enabled: Option<bool>,
    #[param]
    id: Option<String>,
    #[parent]
    parent: Option<&'a DicomUriBuilder<String>>,
}

/// Represents URI paths available for managing
/// DICOM object identifiers.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/identifiers", requires = "id_is_none!()")]
#[match_path(path = "{parent}/identifiers/{bean_id}", requires = "id_is_none!()")]
pub struct IdentifiersUriBuilder<'a> {
    #[param]
    bean_id: Option<String>,
    #[parent]
    parent: Option<&'a DicomScpUriBuilder<'a>>
}

/// Represents URI paths available for managing
/// DICOM SCP AE title and port.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/title/{title}/{port}")]
pub struct TitleUriBuilder<'a> {
    #[param]
    port: Option<u32>,
    #[param]
    title: Option<String>,
    #[parent]
    parent: Option<&'a DicomScpUriBuilder<'a>>
}

impl DicomScpUriBuilder<'_> {
    /// Continue the builder into a
    /// `IdentifiersUriBuilder`.
    pub fn identifiers(&self) -> IdentifiersUriBuilder {
        IdentifiersUriBuilder::from_parent(self)
    }

    /// Continue the builder into a
    /// `TitleUriBuilder`.
    pub fn title(&self) -> TitleUriBuilder {
        TitleUriBuilder::from_parent(self)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum AnonAction {
    Enabled,
    Site,
    SiteEnabled,
    #[default]
    None
}

macro_rules! anon_action_is {
    ($type:ident) => {
        (|this: &Self| this.action == AnonAction::$type)
    };
}

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/anonymize/projects/{project_id}/enabled", requires = "anon_action_is!(Enabled)")]
#[match_path(path = "{parent}/anonymize/projects/{project_id}")]
#[match_path(path = "{parent}/anonymize/site/enabled", requires = "anon_action_is!(SiteEnabled)")]
#[match_path(path = "{parent}/anonymize/site")]
#[match_path(path = "{parent}/anonymize/default")]
pub struct AnonymizeUriBuilder<'a> {
    action: AnonAction,
    #[param]
    project_id: Option<String>,
    #[parent]
    parent: Option<&'a DicomUriBuilder<String>>
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DicomListAction {
    Active,
    All,
    #[default]
    ById,
}

macro_rules! list_action_is {
    ($type:ident) => {
        (|this: &Self| this.action == DicomListAction::$type)
    };
}

/// Represents URI paths available to inspect
/// DICOM import requests.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/dicom/list/{id}", requires = "list_action_is!(ById)")]
#[match_path(path = "{parent}/dicom/list/active", requires = "list_action_is!(Active)")]
#[match_path(path = "{parent}/dicom/list/all", requires = "list_action_is!(All)")]
pub struct ListImportUriBuilder<'a> {
    action: DicomListAction,
    #[param]
    id: Option<String>,
    #[parent]
    parent: Option<&'a DicomUriBuilder<String>>,
}

impl DicomUriBuilder<String> {
    /// Continue the builder into a
    /// `AnonymizeUriBuilder`.
    pub fn anonymize(&self) -> AnonymizeUriBuilder {
        AnonymizeUriBuilder::from_parent(&Arc::new(self))
    }

    /// Continue the builder into a
    /// `ListImportUriBuilder`.
    pub fn list(&self) -> ListImportUriBuilder {
        ListImportUriBuilder::from_parent(&Arc::new(self))
    }

    /// Continue the builder into a
    /// `DicomScpUriBuilder`.
    pub fn scp(&self) -> DicomScpUriBuilder {
        DicomScpUriBuilder::from_parent(&Arc::new(self))
    }
}

/// Legacy URI endpoints for accessing DICOM
/// resource and task information.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/services")]
pub struct DicomUriLegacyBuilder<Parent>
where
    Parent: DicomAdminUriBuilder,
{
    #[parent]
    parent: Option<Arc<Parent>>,
}

/// Legacy DICOM SCP endpoints.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/dicomscp")]
#[match_path(path = "{parent}/dicom/{status}")]
#[match_path(path = "{parent}/settings/enabledDicomReceiver/{default_setting}")]
pub struct DicomScpLegacyUriBuilder<'a> {
    #[param]
    default_setting: Option<String>,
    #[param]
    status: Option<String>,
    #[parent]
    parent: Option<&'a DicomUriLegacyBuilder<String>>
}

impl DicomUriLegacyBuilder<String> {
    /// Produce the scanners URI path.
    pub fn build_scanners(&self) -> crate::BuildResult {
        self.build_join("scanners")
    }

    /// Continue the builder into a
    /// `DicomScpUriLegacyBuilder`.
    pub fn scp(&self) -> DicomScpLegacyUriBuilder {
        DicomScpLegacyUriBuilder::from_parent(&Arc::new(self))
    }
}

/// Represents URI paths available for managing
/// DICOM resources and actions.
pub trait DicomUri: Version {
    /// URI endpoint paths to manage DICOM
    /// resources and tasks.
    #[inline]
    fn dicom(&self) -> DicomUriBuilder<String> {
        DicomUriBuilder::from_parent(self.root_uri().into())
    }
}

/// Represents legacy URI paths for managing DICOM
/// resources and actions.
pub trait DicomUriLegacy: Version {
    #[inline]
    fn dicom_legacy(&self) -> DicomUriLegacyBuilder<String> {
        DicomUriLegacyBuilder::from_parent(self.data_uri().into())
    }
}
