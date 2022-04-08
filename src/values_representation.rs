use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ValuesRepresentation {
    pub outputs: HashMap<String, Output>,
    pub root_module: Module,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub value: serde_json::Value,
    pub sensitive: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Module {
    /// `None` for the root module
    pub address: Option<String>,
    pub resources: Vec<Resource>,
    pub child_modules: Option<Vec<Module>>,
}

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub address: String,
    pub mode: Mode,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub name: String,
    pub index: Option<serde_json::Value>,
    pub provider_name: String,
    pub schema_version: i8,
    pub values: HashMap<String, serde_json::Value>,
    pub sensitive_values: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Managed,
    Data,
}
