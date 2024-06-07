use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PipelineConfig {
    #[serde(rename = "Applies To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<String>,
    #[serde(rename = "Datatype")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datatype: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Generates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generates: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
}