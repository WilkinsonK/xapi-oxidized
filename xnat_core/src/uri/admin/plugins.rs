use std::{fmt::Debug, sync::Arc};

use oxinat_derive::uri_builder_alias;

use crate::{UriBuilder, Version};

uri_builder_alias!(PluginAdminUriBuilder);
ImplPluginAdminUriBuilder! {
    (String),
}

/// Represents the API endpoints available for
/// listing plugins installed on an XNAT.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/plugins")]
#[match_path(path = "{parent}/plugins/{plugin}")]
pub struct PluginsUriBuilder<Parent>
where
    Parent: PluginAdminUriBuilder,
{
    #[param]
    plugin: Option<String>,
    #[parent]
    parent: Option<Arc<Parent>>,
}

/// Represent the URI paths available for
/// management of plugins installed on an XNAT.
pub trait PluginUri: Version {
    /// URI endpoints for accessing installed
    /// plugins.
    #[inline]
    fn plugins(&self) -> PluginsUriBuilder<String> {
        PluginsUriBuilder::from_parent(self.root_uri().into())
    }

    /// URI endpoint to access plugin settings.
    #[inline]
    fn plugin_settings(&self) -> crate::BuildResult {
        self.root_uri().build_join("pluginOpenUrls/settings")
    }
}
