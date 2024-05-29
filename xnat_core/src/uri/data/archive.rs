use std::{fmt::Debug, path::PathBuf, sync::Arc};

use oxinat_derive::uri_builder_alias;

use crate::{UriBuilder, Version};

uri_builder_alias!(ArchiveDataUriBuilder);
ImplArchiveDataUriBuilder! {
    (String),
}
ImplArchiveDataUriBuilder! {
    (PrearchiveUriBuilder<Parent>, Parent),
    (PrearchProjectUriBuilder<'_, Parent>, Parent),
    (PrearchScanUriBuilder<'_, Parent>, Parent),
}

/// Represents XNAT archive resource URI
/// endpoints.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "")] // Passthrough.
pub struct ArchiveUriBuilder<Parent>
where
    Parent: Version,
{
    #[parent]
    parent: Option<Arc<Parent>>
}

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/services")]
pub struct ServicesUriBuilder<Parent>
where
    Parent: ArchiveDataUriBuilder,
{
    #[parent]
    parent: Option<Arc<Parent>>
}

impl ServicesUriBuilder<String> {
    /// Produce the services/archive URI endpoint.
    pub fn build_archive(&self) -> crate::BuildResult {
        self.build_join("archive")
    }

    /// Produce the services/import URI endpoint.
    pub fn build_import(&self) -> crate::BuildResult {
        self.build_join("import")
    }

    /// Produce the services/dicomdump URI
    /// endpoint.
    pub fn build_dicomdump(&self) -> crate::BuildResult {
        self.build_join("dicomdump")
    }

    /// Produce the services/validate-archive
    /// URI endpoint.
    pub fn build_validate_archive(&self) -> crate::BuildResult {
        self.build_join("validate-archive")
    }
}

/// Represents URI endpoints for managing XNAT
/// prearchive data.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/prearchive")]
pub struct PrearchiveUriBuilder<Parent>
where
    Parent: ArchiveDataUriBuilder,
{
    #[parent]
    parent: Option<Arc<Parent>>
}

/// Represents URI endpoints available for
/// managing projects in the prearchive.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/projects")]
#[match_path(path = "{parent}/projects/{project}")]
pub struct PrearchProjectUriBuilder<'a, Parent>
where
    Parent: ArchiveDataUriBuilder,
{
    #[param]
    project: Option<String>,
    #[parent]
    parent: Option<&'a Parent>
}

/// Represents URI endpoints available for
/// managing scans of some project in the
/// prearchive.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/{timestamp}/{experiment}/scans")]
#[match_path(path = "{parent}/{timestamp}/{experiment}/scans/{scan}")]
pub struct PrearchScanUriBuilder<'a, Parent>
where
    Parent: ArchiveDataUriBuilder,
{
    #[param]
    experiment: Option<String>,
    #[param]
    scan: Option<String>,
    #[param]
    timestamp: Option<u64>,
    #[parent]
    parent: Option<&'a Parent>
}

/// Represents the URI endpoints available for
/// managing prearchived scan resources.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/resources")]
#[match_path(path = "{parent}/resources/{resource}")]
#[match_path(path = "{parent}/resources/{resource}/files/{file}")]
pub struct PrearchResourceUriBuilder<'a, Parent>
where
    Parent: ArchiveDataUriBuilder,
{
    #[param(map_from = "|pb: &PathBuf| pb.to_str().unwrap().to_string()")]
    file: Option<PathBuf>,
    #[param]
    resource: Option<String>,
    #[parent]
    parent: Option<&'a Parent>
}

impl<Parent: ArchiveDataUriBuilder> PrearchResourceUriBuilder<'_, Parent> {
    /// Produce the resources/{resource}/files
    /// URI endpoint.
    pub fn build_files(&self) -> crate::BuildResult {
        self.build_join_if("files", |s| {
            s.resource.is_some() && s.file.is_none()
        })
    }
}

impl<Parent> PrearchScanUriBuilder<'_, Parent>
where
    Parent: ArchiveDataUriBuilder + Default,
{
    /// Continue the builder into a
    /// `PrearchResourceUriBuilder`.
    pub fn resources(&self) -> PrearchResourceUriBuilder<'_, Self> {
        PrearchResourceUriBuilder::from_parent(self)
    }
}

impl<Parent> PrearchProjectUriBuilder<'_, Parent>
where
    Parent: ArchiveDataUriBuilder + Default
{
    /// Continue the builder into a
    /// `PrearchScanUriBuilder`.
    pub fn scans(&self) -> PrearchScanUriBuilder<'_, Self> {
        PrearchScanUriBuilder::from_parent(self)
    }
}

impl<Parent: ArchiveDataUriBuilder + Default> PrearchiveUriBuilder<Parent> {
    /// Produce the prearchive/delete URI
    /// endpoint.
    pub fn build_delete(&self) -> crate::BuildResult {
        self.build_join("delete")
    }

    /// Produce the prearchive/move URI endpoint.
    pub fn build_move(&self) -> crate::BuildResult {
        self.build_join("move")
    }

    /// Produce the prearchive/rebuild URI
    /// endpoint.
    pub fn build_rebuild(&self) -> crate::BuildResult {
        self.build_join("rebuild")
    }

    /// Continue the builder into a
    /// `PrearchProjectUriBuilder`.
    pub fn projects(&self) -> PrearchProjectUriBuilder<'_, Self> {
        PrearchProjectUriBuilder::from_parent(self)
    }
}

/// Represnts URI endpoints available to monitor
/// the status of an upload.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/status/{upload_id}")]
pub struct UploadStatusUriBuilder<Parent>
where
    Parent: ArchiveDataUriBuilder,
{
    #[param]
    upload_id: Option<String>,
    #[parent]
    parent: Option<Arc<Parent>>
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ProcessorOpt {
    CanRemap,
    Classes,
    Create,
    Enabled,
    Id,
    List,
    Summary,
    #[default]
    None,
}

macro_rules! is_proc_opt {
    ($type:ident) => {
        (|this: &Self| this.processor_option == ProcessorOpt::$type)
    };
}

/// Represents URI endpoints available to manage
/// installed archive processors.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/processors")]
#[match_path(path = "{parent}/processors/classes", requires = "is_proc_opt!(Classes)")]
#[match_path(path = "{parent}/processors/site/canRemap/receiver/{ae_port}", requires = "is_proc_opt!(CanRemap)")]
#[match_path(path = "{parent}/processors/site/create", requires = "is_proc_opt!(Create)")]
#[match_path(path = "{parent}/processors/site/enabled/receiver/{ae_port}", requires = "is_proc_opt!(Enabled)")]
#[match_path(path = "{parent}/processors/site/enabled", requires = "is_proc_opt!(Enabled)")]
#[match_path(path = "{parent}/processors/site/enabled/summary", requires = "is_proc_opt!(Summary)")]
#[match_path(path = "{parent}/processors/site/id/{instance_id}")]
#[match_path(path = "{parent}/processors/list", requires = "is_proc_opt!(List)")]
pub struct ProcessorsUriBuilder<Parent>
where
    Parent: ArchiveDataUriBuilder,
{
    processor_option: ProcessorOpt,

    #[param]
    ae_port: Option<u64>,
    #[param]
    instance_id: Option<String>,
    #[parent]
    parent: Option<Arc<Parent>>,
}

/// Represents URI paths to access DICOM mapping
/// endpoints.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/dicom_mappings")]
#[match_path(path = "{parent}/dicom_mappings/{id}")]
pub struct DicomMappingUriBuilder<Parent>
where
    Parent: ArchiveDataUriBuilder,
{
    #[param]
    id: Option<String>,
    #[parent]
    parent: Option<Arc<Parent>>
}

impl<Parent> DicomMappingUriBuilder<Parent>
where
    Parent: ArchiveDataUriBuilder
{
    /// Produces the dicom_mappings/update URI
    /// path.
    pub fn build_update(&self) -> crate::BuildResult {
        self.build_join_if("update", |s| s.id.is_none())
    }
}

impl<Parent: Version> ArchiveUriBuilder<Parent> {
    /// Continue the builder into a
    /// `DicomMappingUriBuilder`.
    pub fn dicom_mappings(&self) -> DicomMappingUriBuilder<String> {
        DicomMappingUriBuilder::from_parent(self.root_uri())
    }

    /// Continue the builder into a
    /// `PrearchiveUriBuilder`.
    pub fn prearchive(&self) -> PrearchiveUriBuilder<String> {
        PrearchiveUriBuilder::from_parent(self.data_uri())
    }

    /// Continue the builder into a
    /// `ProcessorsUriBuilder`.
    pub fn processors(&self) -> ProcessorsUriBuilder<String> {
        ProcessorsUriBuilder::from_parent(self.root_uri())
    }

    /// Continue the builder into a
    /// `ServicesUriBuilder`.
    pub fn services(&self) -> ServicesUriBuilder<String> {
        ServicesUriBuilder::from_parent(self.data_uri())
    }

    /// Continue the builder into a
    /// `UploadStatusUriBuilder`.
    pub fn upload_status(&self) -> UploadStatusUriBuilder<String> {
        UploadStatusUriBuilder::from_parent(self.data_uri())
    }

    fn data_uri(&self) -> Arc<String> {
        self.parent.as_ref().unwrap().data_uri().into()
    }

    fn root_uri(&self) -> Arc<String> {
        self.parent.as_ref().unwrap().root_uri().into()
    }
}

/// Represents URI paths available to an XNAT user
/// to manage archive resources.
pub trait ArchiveUri: Clone + Default + Sized + Version {
    /// Represents XNAT archive resource URI
    /// endpoints.
    #[inline]
    fn archive_data(&self) -> ArchiveUriBuilder<Self> {
        ArchiveUriBuilder::from_parent(self.to_owned().into())
    }
}
