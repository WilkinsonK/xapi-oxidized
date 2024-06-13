use serde::{Deserialize, Serialize};

use crate::models::common::FormatSpecifier;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames: Option<u64>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_description: Option<String>,
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xnat_imagescandata_id: Option<u64>,
    #[serde(rename = "xsiType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xsi_type: Option<String>,

    // Read-only fields not meant for only for the
    // host to modify.
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insert_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insert_user: Option<String>,

    // Extra query specifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<FormatSpecifier>,

    // Additional data that can be utilized at
    // runtime.
    #[serde(skip_serializing)]
    pub experiment: Option<String>,
    #[serde(skip_serializing)]
    pub project: Option<String>,
    #[serde(skip_serializing)]
    pub subject: Option<String>,
}

impl Scan {
    /// Get READ-ONLY last-modified datetime.
    pub fn last_modified(&self) -> &Option<String> {
        &self.last_modified
    }

    /// Get READ-ONLY insert-date datetime.
    pub fn insert_date(&self) -> &Option<String> {
        &self.insert_date
    }

    /// Get READ-ONLY insert-user name.
    pub fn insert_user(&self) -> &Option<String> {
        &self.insert_user
    }
}
