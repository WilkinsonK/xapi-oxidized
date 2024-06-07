use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use super::{Automation, Notifications, SiteConfig};

#[derive(Debug, Deserialize, Serialize)]
pub struct Preferences {
    #[serde(rename = "autmation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<Automation>,
    #[serde(rename = "dicomScpManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dicom_scp_manager: Option<HashMap<String, String>>,
    #[serde(rename = "notifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Notifications>,
    #[serde(rename = "siteConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_config: Option<SiteConfig>,
    #[serde(flatten)]
    extra: HashMap<String, Value>
}

impl Preferences {
    /// Attempts to get and parse some metadata
    /// into a model from the extra serialized
    /// data map.
    pub fn get_extra<T>(&self, name: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        serde_json::from_value(self.extra.get(name)?.clone()).ok()
    }
}
