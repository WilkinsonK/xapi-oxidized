use std::borrow::Cow;
use std::fmt::Debug;
use std::path::PathBuf;
use std::rc::Rc;

use oxinat_derive::UriBuilder;

use crate::pathbuf_to_string;
use crate::uri::UriBuilder;
use crate::version::Version;

/// Represents the URI paths available for
/// endpoints meant for interacting with
/// data types installed on some XNAT.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/schemas")]
#[match_path(path = "{parent}/schemas/{schema}")]
#[match_path(path = "{parent}/{namespace}/{schema}")]
pub struct SchemaUriBuilder<'a, Parent>
where
    Parent: UriBuilder + Clone + Debug,
{
    #[param]
    schema: Option<Cow<'a, str>>,
    #[param]
    namespace: Option<Cow<'a, str>>,
    #[parent]
    parent: Option<Rc<Parent>>
}

/// Represents the URI paths available for
/// endpoints meant for interacting with an XNAT
/// site-wide server properties.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/siteConfig")]
#[match_path(path = "{parent}/siteConfig/{property}")]
pub struct SiteConfigUriBuilder<'a, Parent>
where
    Parent: UriBuilder + Clone + Debug,
{
    #[param]
    property: Option<Cow<'a, str>>,
    #[parent]
    parent: Option<Rc<Parent>>
}

/// Represents and builds URI endpoints for
/// acquiring build properties.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/buildInfo")]
#[match_path(path = "{parent}/buildInfo/{property}")]
pub struct BuildInfoUriBuilder<'a, Parent>
where
    Parent: UriBuilder + Clone + Debug,
{
    #[param]
    property: Option<Cow<'a, str>>,
    #[parent]
    parent: Option<Rc<Parent>>
}

impl BuildInfoUriBuilder<'_, SiteConfigUriBuilder<'_, String>> {
    /// Produce the
    /// siteConfig/buildInfo/attributes URI
    /// endpoint.
    pub fn attributes(&self) -> anyhow::Result<String> {
        self.build_join(String::from("attributes"))
    }
}

/// Represents and builds URI endpoints for
/// getting system uptime.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/uptime")]
pub struct UptimeUriBuilder<Parent>
where
    Parent: UriBuilder + Clone + Debug,
{
    #[parent]
    parent: Option<Rc<Parent>>
}

impl UptimeUriBuilder<SiteConfigUriBuilder<'_, String>> {
    /// Produce the siteConfig/uptime/display
    /// URI endpoint.
    pub fn display(&self) -> anyhow::Result<String> {
        self.build_join(String::from("display"))
    }
}

/// Represents and build URI endpoints for
/// acquiring an XNAT's configuration properties.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/values/{preferences}")]
pub struct ValuesUriBuilder<'a, Parent>
where
    Parent: UriBuilder + Clone + Debug,
{
    #[param]
    preferences: Option<Cow<'a, str>>,
    #[parent]
    parent: Option<Rc<Parent>>
}

impl<Parent> SiteConfigUriBuilder<'_, Parent>
where
    Parent: UriBuilder + Clone + Debug + Default,
    Self: UriBuilder + Clone + Debug,
{
    /// Continue the builder into a
    /// `BuildInfoUriBuilder`.
    pub fn build_info(&self) -> BuildInfoUriBuilder<Self> {
        // TODO: fix this clone nightmare.
        BuildInfoUriBuilder::from_parent(self.clone().into())
    }

    /// Continue the builder into a
    /// `UptimeUriBuilder`.
    pub fn uptime(&self) -> UptimeUriBuilder<Self> {
        UptimeUriBuilder::from_parent(self.clone().into())
    }

    /// Produce a siteConfig/values/{pref} URI
    /// endpoint.
    pub fn values(&self, pref: &str) -> anyhow::Result<String> {
        ValuesUriBuilder::from_parent(self.clone().into())
            .with_preferences(&pref.into())
            .build()
    }
}

/// Represents the URI paths available for
/// endpoints meant for interacting with an XNAT
/// internal functions for remote clients.
#[derive(Debug,  Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/prefs")]
pub struct PreferenceUriBuilder<Parent>
where
    Parent: UriBuilder + Clone + Debug,
{
    #[parent]
    parent: Option<Rc<Parent>>,
}

/// Represents the URI paths available to a user
/// to acquire a map of preferences and values for
/// an XNAT.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/ini")]
#[match_path(path = "{parent}/ini/{tool_id}")]
pub struct IniUriBuilder<'a, Parent>
where
    Parent: UriBuilder + Clone + Debug,
{
    #[param]
    tool_id: Option<Cow<'a, str>>,
    #[parent]
    parent: Option<Rc<Parent>>
}

/// Represents the URI paths available to a user
/// to manage preferences and values for an XNAT.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/props")]
#[match_path(path = "{parent}/props/{tool_id}")]
pub struct PropsUriBuilder<'a, Parent>
where
    Parent: UriBuilder + Clone + Debug,
{
    #[param]
    tool_id: Option<Cow<'a, str>>,
    #[parent]
    parent: Option<Rc<Parent>>
}

impl<Parent> PreferenceUriBuilder<Parent>
where
    Parent: UriBuilder + Clone + Debug + Default,
    Self: UriBuilder + Clone + Debug,
{
    /// Continue the builder into a
    /// `IniUriBuilder`.
    pub fn ini(&self) -> IniUriBuilder<Self> {
        IniUriBuilder::from_parent(self.clone().into())
    }

    /// Continue the builder into a
    /// `PropsUriBuilder`.
    pub fn properties(&self) -> PropsUriBuilder<Self> {
        PropsUriBuilder::from_parent(self.clone().into())
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
    fn config(&self) -> String {
        let mut uri = PathBuf::from(self.root_uri());
        uri.push("config");
        pathbuf_to_string(&uri)
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

        let b = b.with_schema(&Cow::from("phoney_schema"));
        assert_eq!(b.build().unwrap(), String::from("/schemas/phoney_schema"));

        let b = b.with_namespace(&Cow::from("phoney_namespace"));
        assert_eq!(b.build().unwrap(), String::from("/phoney_namespace/phoney_schema"))
    }
}
