use serde::{Deserialize, Serialize};

use crate::{get_from_datafields, models::common::{FormatSpecifier, Item}};

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

impl Item<Scan> {
    pub fn condition(&self) -> &Option<String> {
        get_from_datafields!(self, condition)
    }

    pub fn documentation(&self) -> &Option<String> {
        get_from_datafields!(self, documentation)
    }

    pub fn frames(&self) -> &Option<u64> {
        get_from_datafields!(self, frames)
    }

    pub fn id(&self) -> &Option<u64> {
        get_from_datafields!(self, id)
    }

    pub fn modality(&self) -> &Option<String> {
        get_from_datafields!(self, modality)
    }

    pub fn note(&self) -> &Option<String> {
        get_from_datafields!(self, note)
    }

    pub fn quality(&self) -> &Option<String> {
        get_from_datafields!(self, quality)
    }

    pub fn scan_type(&self) -> &Option<u8> {
        get_from_datafields!(self, scan_type)
    }

    pub fn series_description(&self) -> &Option<String> {
        get_from_datafields!(self, series_description)
    }

    pub fn uid(&self) -> &Option<String> {
        get_from_datafields!(self, uid)
    }

    pub fn uri(&self) -> &Option<String> {
        get_from_datafields!(self, uri)
    }

    pub fn validation_date(&self) -> &Option<String> {
        get_from_datafields!(self, validation_date)
    }

    pub fn validation_method(&self) -> &Option<String> {
        get_from_datafields!(self, validation_method)
    }

    pub fn validation_notes(&self) -> &Option<String> {
        get_from_datafields!(self, validation_notes)
    }

    pub fn validation_status(&self) -> &Option<String> {
        get_from_datafields!(self, validation_status)
    }

    pub fn xnat_imagescandata_id(&self) -> &Option<u64> {
        get_from_datafields!(self, xnat_imagescandata_id)
    }

    pub fn last_modified(&self) -> Option<String> {
        self.data_fields.last_modified.clone()
    }

    pub fn insert_date(&self) -> Option<String> {
        self.data_fields.insert_date.clone()
    }

    pub fn insert_user(&self) -> Option<String> {
        self.data_fields.insert_user.clone()
    }
}
