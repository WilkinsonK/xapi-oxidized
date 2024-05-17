use std::fmt::Debug;
use std::path::PathBuf;

use oxinat_derive::UriBuilder;

use crate::pathbuf_to_string;
use crate::uri::UriBuilder;
use crate::version::Version;

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/schemas")]
#[match_path(path = "{parent}/schemas/{schema}")]
#[match_path(path = "{parent}/{namespace}/{schema}")]
pub struct SchemaUriBuilder<Parent>
where
    Parent: UriBuilder + Clone + Debug,
{
    #[param]
    schema: Option<String>,
    #[param]
    namespace: Option<String>,
    #[parent]
    parent: Option<Parent>
}

/// Represents the URI paths available for
/// endpoints meant for interacting with an XNAT
/// site-wide server properties.
#[derive(Debug, Default, Clone, UriBuilder)]
#[match_path(path = "{parent}/siteConfig")]
pub struct SiteConfigUriBuilder<Parent>
where
    Parent: UriBuilder + Clone + Debug,
{
    #[parent]
    parent: Option<Parent>
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
    parent: Option<Parent>,
}

/// Represents the URI paths available for
/// endpoints meant for interacting with an XNAT
/// administration features.
pub trait AdminUri: Version {
    /// URI endpoint to access site-wide
    /// configuration.
    #[inline]
    fn site_config(&self) -> SiteConfigUriBuilder<String> {
        SiteConfigUriBuilder::default().with_parent(self.root_uri())
    }
    /// URI endpoint to access preferences.
    #[inline]
    fn preferences(&self) -> PreferenceUriBuilder<String> {
        PreferenceUriBuilder::default().with_parent(self.root_uri())
    }
    /// URI endpoint to access schema metadata.
    #[inline]
    fn schema(&self) -> SchemaUriBuilder<String> {
        SchemaUriBuilder::default().with_parent(self.root_uri())
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
        let b = SchemaUriBuilder::default()
        .with_parent("".to_string());
        assert_eq!(b.build().unwrap(), String::from("/schemas"));

        let b = b.with_schema(&String::from("phoney_schema"));
        assert_eq!(b.build().unwrap(), String::from("/schemas/phoney_schema"));

        let b = b.with_namespace(&String::from("phoney_namespace"));
        assert_eq!(b.build().unwrap(), String::from("/phoney_namespace/phoney_schema"))
    }
}
