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
    Parent: SysUriBuilder,
    Self: SysUriBuilder + Default,
{
    /// Continue the builder into a
    /// `RefreshUriBuilder`.
    pub fn refresh(&self) -> RefreshUriBuilder<Self> {
        RefreshUriBuilder::from_parent(self.clone().into())
    }
}

impl<Parent> ArchiveUriBuilder<Parent>
where
    Parent: SysUriBuilder + Default,
{
    pub fn catalog(&self) -> CatalogsUriBuilder<Self> {
        CatalogsUriBuilder::from_parent(self.clone().into())
    }
}

/// Represent the URI paths available for
/// endpoints meant for interacting with an XNAT
/// archive catalog.
pub trait SysUri: Version {
    /// URI endpoint to access the archive catalog
    /// API.
    #[inline]
    fn archive(&self) -> ArchiveUriBuilder<String> {
        ArchiveUriBuilder::from_parent(self.root_uri().into())
    }
}
