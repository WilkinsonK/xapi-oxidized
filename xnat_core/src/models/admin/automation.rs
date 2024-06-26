//! Generated via `helpers/gen_models.py`
//! Do not edit this file directly.
//! 
//! Defines models and model units available to a
//! user for manipulating `./automation` values.

use serde::{Deserialize, Serialize};

use oxinat_derive::ModelField;

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "internalScriptingEnabled")]
pub struct InternalScriptingEnabled(bool);

#[derive(Debug, Deserialize, Serialize)]
pub struct Automation {
    #[serde(rename = "internalScriptingEnabled")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_scripting_enabled: Option<InternalScriptingEnabled>,
}

