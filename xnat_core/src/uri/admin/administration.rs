use std::fmt::Debug;
use std::sync::Arc;

use oxinat_derive::{uri_builder_alias, UriBuilder};

use crate::uri::UriBuilder;
use crate::version::Version;

uri_builder_alias!(AdminUriBuilder);
// Requires no generics for parent or otherwise.
ImplAdminUriBuilder! {
    (String),
    (DataTypesUriBuilder<'_>),
    (ElementsUriBuilder<'_>),
    (NamesUriBuilder<'_>),
    (BuildInfoUriBuilder<'_>),
    (UptimeUriBuilder<'_>),
    (ValuesUriBuilder<'_>),
    (IniUriBuilder<'_>),
    (PropsUriBuilder<'_>)
}
// Requires generics for parent.
ImplAdminUriBuilder! {
    (SchemaUriBuilder<Parent>, Parent),
    (SiteConfigUriBuilder<Parent>, Parent),
    (SiteConfigUriBuilderLegacy<Parent>, Parent),
    (PreferenceUriBuilder<Parent>, Parent),
}

/// Represents the URI paths available for
/// endpoints meant for interacting with
/// data types installed on some XNAT.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/schemas")]
#[match_path(path = "{parent}/schemas/{schema}")]
#[match_path(path = "{parent}/{namespace}/{schema}")]
pub struct SchemaUriBuilder<Parent>
where
    Parent: AdminUriBuilder,
{
    #[param]
    schema: Option<String>,
    #[param]
    namespace: Option<String>,
    #[parent]
    parent: Option<Arc<Parent>>
}

/// Represents the URI path to items concerning
/// data types. Allowing for a user to access
/// data type information from some XNAT.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/datatypes")]
pub struct DataTypesUriBuilder<'a>
{
    #[parent]
    parent: Option<&'a SchemaUriBuilder<String>>,
}

/// Represents the URI path to items concerning
/// data types installed on an XNAT system.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/elements")]
#[match_path(path = "{parent}/elements/{data_type}")]
pub struct ElementsUriBuilder<'a>
{
    #[param]
    data_type: Option<String>,
    #[parent]
    parent: Option<&'a DataTypesUriBuilder<'a>>,
}

impl ElementsUriBuilder<'_>
{
    /// Produce the
    /// schemas/datatypes/all URI endpoint.
    pub fn build_all(&self) -> anyhow::Result<String> {
        self.build_join("all")
    }
}

/// Represents the URI path to items concerning
/// data type element names and types for
/// specific data.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/names")]
#[match_path(path = "{parent}/names/{data_type}")]
pub struct NamesUriBuilder<'a>
{
    #[param]
    data_type: Option<String>,
    #[parent]
    parent: Option<&'a DataTypesUriBuilder<'a>>,
}

impl NamesUriBuilder<'_>
{
    /// Produce the schemas/names/all URI
    /// endpoint.
    pub fn build_all(&self) -> anyhow::Result<String> {
        self.build_join("all")
    }
}

impl DataTypesUriBuilder<'_>
{
    /// Continue the builder into a data type
    /// `ElementsUriBuilder`.
    pub fn elements(&self) -> ElementsUriBuilder {
        ElementsUriBuilder::from_parent(&Arc::new(self))
    }

    /// Continue the builder into a data type
    /// `NamesUriBuilder`.
    pub fn names(&self) -> NamesUriBuilder {
        NamesUriBuilder::from_parent(&Arc::new(self))
    }
}

impl SchemaUriBuilder<String>
{
    /// Continue the builder into a
    /// `DataTypesUriBuilder`.
    pub fn datatypes(&self) -> DataTypesUriBuilder {
        DataTypesUriBuilder::from_parent(&Arc::new(self))
    }
}

/// Represents the URI paths available for
/// endpoints meant for interacting with an XNAT
/// site-wide server properties.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/siteConfig")]
#[match_path(path = "{parent}/siteConfig/{property}")]
pub struct SiteConfigUriBuilder<Parent>
where
    Parent: AdminUriBuilder,
{
    #[param]
    property: Option<String>,
    #[parent]
    parent: Option<Arc<Parent>>
}

/// Represents and builds URI endpoints for
/// acquiring build properties.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/buildInfo")]
#[match_path(path = "{parent}/buildInfo/{property}")]
pub struct BuildInfoUriBuilder<'a>
{
    #[param]
    property: Option<String>,
    #[parent]
    parent: Option<&'a SiteConfigUriBuilder<String>>
}

impl BuildInfoUriBuilder<'_>
{
    /// Produce the
    /// siteConfig/buildInfo/attributes URI
    /// endpoint.
    pub fn build_attributes(&self) -> anyhow::Result<String> {
        self.build_join("attributes")
    }
}

/// Represents and builds URI endpoints for
/// getting system uptime.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/uptime")]
pub struct UptimeUriBuilder<'a>
{
    #[parent]
    parent: Option<&'a SiteConfigUriBuilder<String>>
}

impl UptimeUriBuilder<'_>
{
    /// Produce the siteConfig/uptime/display
    /// URI endpoint.
    pub fn build_display(&self) -> anyhow::Result<String> {
        self.build_join("display")
    }
}

/// Represents and build URI endpoints for
/// acquiring an XNAT's configuration properties.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/values/{preferences}")]
pub struct ValuesUriBuilder<'a>
{
    #[param]
    preferences: Option<String>,
    #[parent]
    parent: Option<&'a SiteConfigUriBuilder<String>>
}

impl SiteConfigUriBuilder<String>
{
    /// Continue the builder into a
    /// `BuildInfoUriBuilder`.
    pub fn build_info(&self) -> BuildInfoUriBuilder {
        BuildInfoUriBuilder::from_parent(&Arc::new(self))
    }

    /// Continue the builder into a
    /// `UptimeUriBuilder`.
    pub fn uptime(&self) -> UptimeUriBuilder {
        UptimeUriBuilder::from_parent(&Arc::new(self))
    }

    /// Produce a siteConfig/values/{pref} URI
    /// endpoint.
    pub fn values(&self, pref: &str) -> anyhow::Result<String> {
        ValuesUriBuilder::from_parent(&Arc::new(self))
            .with_preferences(pref)
            .build()
    }
}

#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/config")]
#[match_path(path = "{parent}/config/{tool_id}")]
#[match_path(path = "{parent}/config/{tool_id}/{file_path}")]
pub struct SiteConfigUriBuilderLegacy<Parent>
where
    Parent: AdminUriBuilder,
{
    #[param]
    file_path: Option<String>,
    #[param]
    tool_id: Option<String>,
    #[parent]
    parent: Option<Arc<Parent>>
}

/// Represents the URI paths available for
/// endpoints meant for interacting with an XNAT
/// internal functions for remote clients.
#[derive(Debug,  Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/prefs")]
pub struct PreferenceUriBuilder<Parent>
where
    Parent: AdminUriBuilder,
{
    #[parent]
    parent: Option<Arc<Parent>>,
}

/// Represents the URI paths available to a user
/// to acquire a map of preferences and values for
/// an XNAT.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/ini")]
#[match_path(path = "{parent}/ini/{tool_id}")]
pub struct IniUriBuilder<'a>
{
    #[param]
    tool_id: Option<String>,
    #[parent]
    parent: Option<&'a PreferenceUriBuilder<String>>
}

/// Represents the URI paths available to a user
/// to manage preferences and values for an XNAT.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/props")]
#[match_path(path = "{parent}/props/{tool_id}")]
pub struct PropsUriBuilder<'a>
{
    #[param]
    tool_id: Option<String>,
    #[parent]
    parent: Option<&'a PreferenceUriBuilder<String>>
}

impl PreferenceUriBuilder<String>
{
    /// Continue the builder into a
    /// `IniUriBuilder`.
    pub fn ini(&self) -> IniUriBuilder {
        IniUriBuilder::from_parent(&Arc::new(self))
    }

    /// Continue the builder into a
    /// `PropsUriBuilder`.
    pub fn properties(&self) -> PropsUriBuilder {
        PropsUriBuilder::from_parent(&Arc::new(self))
    }
}

/// Represents the URI paths available for
/// endpoints meant for interacting with an XNAT
/// administration features.
pub trait AdminUri: Version {
    /// URI endpoint to access site-wide
    /// configuration.
    #[inline]
    fn site_config(&self) -> SiteConfigUriBuilder<String> {
        SiteConfigUriBuilder::from_parent(self.root_uri().into())
    }
    /// URI endpoint to access preferences.
    #[inline]
    fn preferences(&self) -> PreferenceUriBuilder<String> {
        PreferenceUriBuilder::from_parent(self.root_uri().into())
    }
    /// URI endpoint to access schema metadata.
    #[inline]
    fn schema(&self) -> SchemaUriBuilder<String> {
        SchemaUriBuilder::from_parent(self.root_uri().into())
    }
}

/// Represents the legacy implementation for
/// administrative URI endpoints.
pub trait AdminUriLegacy: Version {
    /// URI endpoint to access site configuration.
    #[inline]
    fn config(&self) -> SiteConfigUriBuilderLegacy<String> {
        SiteConfigUriBuilderLegacy::from_parent(self.data_uri().into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_schema_can_build_uri() {
        let b = SchemaUriBuilder::<String>::default()
        .with_parent("".to_string().into());
        assert_eq!(b.build().unwrap(), String::from("/schemas"));

        let b = b.with_schema(String::from("phoney_schema"));
        assert_eq!(b.build().unwrap(), String::from("/schemas/phoney_schema"));

        let b = b.with_namespace(String::from("phoney_namespace"));
        assert_eq!(b.build().unwrap(), String::from("/phoney_namespace/phoney_schema"))
    }
}
