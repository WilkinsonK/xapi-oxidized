use serde::{Deserialize, Serialize};

use crate::{get_from_datafields, models::common::{FormatSpecifier, Item}};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scan {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_description: Option<String>,
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xnat_imagescandata_id: Option<u64>,
    #[serde(rename = "xsiType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xsi_type: Option<String>,

    // Extra query specifiers
    pub format: Option<FormatSpecifier>,
}

impl Item<Scan> {
    pub fn id(&self) -> &Option<u64> {
        get_from_datafields!(self, id)
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

    pub fn uri(&self) -> &Option<String> {
        get_from_datafields!(self, uri)
    }

    pub fn xnat_imagescandata_id(&self) -> &Option<u64> {
        get_from_datafields!(self, xnat_imagescandata_id)
    }
}
