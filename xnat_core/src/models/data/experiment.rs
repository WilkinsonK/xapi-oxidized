use serde::{Deserialize, Serialize};

use super::scan::Scan;
use crate::{get_from_datafields, models::common::{FormatSpecifier, Item}};

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

impl Item<Experiment> {
    pub fn visit_id(&self) -> &Option<String> {
        get_from_datafields!(self, visit_id)
    }

    pub fn data(&self) -> &Option<String> {
        get_from_datafields!(self, date)
    }

    pub fn id(&self) -> &Option<String> {
        get_from_datafields!(self, id)
    }

    pub fn project(&self) -> &Option<String> {
        get_from_datafields!(self, project)
    }

    pub fn label(&self) -> &Option<String> {
        get_from_datafields!(self, label)
    }

    pub fn time(&self) -> &Option<String> {
        get_from_datafields!(self, time)
    }

    pub fn note(&self) -> &Option<String> {
        get_from_datafields!(self, note)
    }

    pub fn pi_firstname(&self) -> &Option<String> {
        get_from_datafields!(self, pi_firstname)
    }

    pub fn pi_lastname(&self) -> &Option<String> {
        get_from_datafields!(self, pi_lastname)
    }

    pub fn uri(&self) -> &Option<String> {
        get_from_datafields!(self, uri)
    }

    pub fn validation_method(&self) -> &Option<String> {
        get_from_datafields!(self, validation_method)
    }

    pub fn validation_status(&self) -> &Option<String> {
        get_from_datafields!(self, validation_status)
    }

    pub fn validation_date(&self) -> &Option<String> {
        get_from_datafields!(self, validation_date)
    }

    pub fn validation_notes(&self) -> &Option<String> {
        get_from_datafields!(self, validation_notes)
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
