use serde::{Deserialize, Serialize};

use super::subject::Subject;
use crate::models::common::FormatSpecifier;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi_firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi_lastname: Option<String>,
    #[serde(rename = "secondary_ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_id: Option<String>,
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    // Extra query specifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<FormatSpecifier>,

    // Additional data that can be utilized at
    // runtime.
    #[serde(skip_serializing)]
    pub subjects: Option<Vec<Subject>>,
}
