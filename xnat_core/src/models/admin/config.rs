use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tool {
    tool: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigLegacy {
    #[serde(flatten)]
    tools: Vec<Tool>
}
