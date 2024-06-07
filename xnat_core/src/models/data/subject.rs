use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use super::experiment::Experiment;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Subject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_weight: Option<f32>,
    #[serde(rename = "dob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education: Option<String>,
    #[serde(rename = "educationDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gestational_age: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handedness: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi_firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi_lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_menstrual_age: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,
    #[serde(rename = "yob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year_of_birth: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    insert_date: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insert_user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified: Option<NaiveDateTime>,

    #[serde(flatten)]
    #[serde(skip_serializing)]
    pub sessions: Vec<Experiment>,
}

impl Subject {
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
