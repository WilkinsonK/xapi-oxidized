use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

use super::scan::Scan;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Experiment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<NaiveTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi_firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi_lastname: Option<String>,
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_date: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_notes: Option<String>,
    #[serde(rename = "xsiType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xsi_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insert_date: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insert_user: Option<String>,

    #[serde(flatten)]
    #[serde(skip_serializing)]
    pub scans: Vec<Scan>,
}

impl Experiment {
    /// Get READ-ONLY last-modified datetime.
    pub fn last_modified(&self) -> &Option<NaiveDateTime> {
        &self.last_modified
    }

    /// Get READ-ONLY insert-date datetime.
    pub fn insert_date(&self) -> &Option<NaiveDateTime> {
        &self.insert_date
    }

    /// Get READ-ONLY insert-user name.
    pub fn insert_user(&self) -> &Option<String> {
        &self.insert_user
    }
}
