use serde::{Deserialize, Serialize};

use super::{Assessor, Resource, Scan};
use crate::models::common::FormatSpecifier;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Experiment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi_firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi_lastname: Option<String>,
    #[serde(rename = "subject_ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_project: Option<String>,
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_notes: Option<String>,
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

    // Extra query specifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<FormatSpecifier>,

    // Additional data that can be utilized at
    // runtime.
    #[serde(skip_serializing)]
    pub scans: Option<Vec<Scan>>,
}

impl Experiment {
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

impl From<Assessor> for Experiment {
    fn from(value: Assessor) -> Self {
        let mut inst = Self::default();
        inst.id.clone_from(&value.session_id);
        inst.label.clone_from(&value.session_label);
        inst.project.clone_from(&value.project);
        inst.subject_label.clone_from(&value.subject);

        inst
    }
}

impl From<Resource> for Experiment {
    fn from(value: Resource) -> Self {
        let mut inst = Self::default();
        inst.label.clone_from(&value.experiment);
        inst.project.clone_from(&value.project);
        inst.subject_label.clone_from(&value.subject);

        inst
    }
}

impl From<Scan> for Experiment {
    fn from(value: Scan) -> Self {
        let mut inst = Self::default();
        inst.label.clone_from(&value.experiment);
        inst.project.clone_from(&value.project);
        inst.subject_label.clone_from(&value.subject);

        inst
    }
}
