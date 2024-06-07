use serde::{Deserialize, Serialize};

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
}
