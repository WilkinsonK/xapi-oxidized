use std::{fmt::Debug, rc::Rc};

use oxinat_derive::uri_builder_alias;

use crate::{UriBuilder, Version};

uri_builder_alias!(SysUriBuilder);
ImplSysUriBuilder! {
    (String),
}
ImplSysUriBuilder! {
    (ArchiveUriBuilder<Parent>, Parent),
    (RefreshUriBuilder<Parent>, Parent),
    (CatalogsUriBuilder<Parent>, Parent),
    (DownloadUriBuilder<Parent>, Parent),
    (NotificationsUriBuilder<Parent>, Parent),
    (NotifyUriBuilder<Parent>, Parent),
}

/// Represents the URI paths available for
/// endpoints meant for interacting with XNAT
/// archive catalogs.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/archive")]
pub struct ArchiveUriBuilder<Parent>
where
    Parent: SysUriBuilder,
{
    #[parent]
    parent: Option<Rc<Parent>>,
}

/// Represents the URI paths available for
/// endpoints meant for doing manipulations
/// against an XNAT archive catalog.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/catalogs")]
pub struct CatalogsUriBuilder<Parent>
where
    Parent: SysUriBuilder,
{
    #[parent]
    parent: Option<Rc<Parent>>
}

/// Represents the URI paths available for
/// endpoints to request a refresh against an
/// XNAT archive catalog.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/refresh")]
#[match_path(path = "{parent}/refresh/{operations}")]
pub struct RefreshUriBuilder<Parent>
where
    Parent: SysUriBuilder,
{
    #[param(map_from=r#"|o: &Vec<_>| o.join(",")"#)]
    operations: Option<Vec<String>>,
    #[parent]
    parent: Option<Rc<Parent>>,
}

impl<Parent> CatalogsUriBuilder<Parent>
where
    Parent: SysUriBuilder + Default,
{
    /// Continue the builder into a
    /// `RefreshUriBuilder`.
    pub fn refresh(&self) -> RefreshUriBuilder<Self> {
        RefreshUriBuilder::from_parent(self.clone().into())
    }
}

/// Represents the URI paths available for
/// endpoints to download the specified catalog.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/download")]
#[match_path(path = "{parent}/download/{catalog_id}")]
pub struct DownloadUriBuilder<Parent>
where
    Parent: SysUriBuilder,
{
    #[param]
    catalog_id: Option<String>,
    #[parent]
    parent: Option<Rc<Parent>>
}

impl<Parent> DownloadUriBuilder<ArchiveUriBuilder<Parent>>
where
    Parent: SysUriBuilder,
{
    /// Produce the
    /// archive/download/{catalog_id}/test URI
    /// endpoint.
    pub fn build_test(&self) -> anyhow::Result<String> {
        self.build_join("test")
    }

    /// Produce the archive/downloadwithsize URI
    /// endpoint.
    pub fn build_with_size(&self) -> anyhow::Result<String> {
        self.parent.as_ref().unwrap().build_join("downloadwithsize")
    }

    /// Produce the
    /// archive/download/{catalog_id}/xml URI
    /// endpoint.
    pub fn build_xml(&self) -> anyhow::Result<String> {
        self.build_join("xml")
    }

    /// Produce the
    /// archive/download/{catalog_id}/zip URI
    /// endpoint.
    pub fn build_zip(&self) -> anyhow::Result<String> {
        self.build_join("zip")
    }
}

impl<Parent> ArchiveUriBuilder<Parent>
where
    Parent: SysUriBuilder + Default,
{
    /// Continue the builder into a
    /// `CatalogsUriBuilder`.
    pub fn catalogs(&self) -> CatalogsUriBuilder<Self> {
        CatalogsUriBuilder::from_parent(self.clone().into())
    }

    /// Continue the builder into a
    /// `DownloadUriBuilder`.
    pub fn download(&self) -> DownloadUriBuilder<Self> {
        DownloadUriBuilder::from_parent(self.clone().into())
    }

    /// Produce the archive/upload/xml URI
    /// endpoint.
    pub fn build_upload_xml(&self) -> anyhow::Result<String> {
        self.build_join("upload/xml")
    }
}

/// Represents the URI endpoint paths available to
/// a user to allow/disallow notifications
/// provided by the user.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/notifications")]
pub struct NotificationsUriBuilder<Parent>
where
    Parent: SysUriBuilder,
{
    #[parent]
    parent: Option<Rc<Parent>>
}

/// Some supported notification type by an XNAT
/// notification system.
#[derive(Clone, Debug, UriBuilder)]
pub enum NotifyType {
    Par,
    Pipeline,
    Registration,
    Transfer,
    Smtp,
    #[match_path(path = "smtp/host/{p0}")]
    SmtpHost(String),
    #[match_path(path = "smtp/password/{p0}")]
    SmtpPassword(String),
    #[match_path(path = "smtp/port/{p0}")]
    SmtpPort(String),
    #[match_path(path = "smtp/property/{p0}")]
    #[match_path(path = "smtp/property/{p0}/{p1}")]
    SmtpProperty(String, Option<String>),
    #[match_path(path = "smtp/protocol/{p0}")]
    SmtpProtocol(String),
    #[match_path(path = "smtp/username/{p0}")]
    SmtpUsername(String),
}

/// Represents URI endpoint paths available for
/// admin notifications to be enabled/disabled.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/notify/{notify_type}")]
pub struct NotifyUriBuilder<Parent>
where
    Parent: SysUriBuilder,
{
    #[param]
    notify_type: Option<NotifyType>,
    #[parent]
    parent: Option<Rc<Parent>>
}

impl<Parent> NotificationsUriBuilder<Parent>
where
    Parent: SysUriBuilder + Default,
{
    pub fn notify(&self) -> NotifyUriBuilder<Self> {
        NotifyUriBuilder::from_parent(self.clone().into())
    }
}

/// Represent the URI paths available for
/// endpoints meant for interacting with an XNAT
/// archive catalog.
pub trait SystemUri: Version {
    /// URI endpoint to access the archive catalog
    /// API.
    #[inline]
    fn archive(&self) -> ArchiveUriBuilder<String> {
        ArchiveUriBuilder::from_parent(self.root_uri().into())
    }
         
    /// URI endpoint to interact with XNAT
    /// notifications API.
    #[inline]
    fn notifications(&self) -> NotificationsUriBuilder<String> {
        NotificationsUriBuilder::from_parent(self.root_uri().into())
    }
}
