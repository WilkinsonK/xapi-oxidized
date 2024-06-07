use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DicomSCP {
    #[serde(rename = "aeTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ae_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DicomSCPs {
    #[serde(flatten)]
    pub entities: Vec<DicomSCP>,
}
