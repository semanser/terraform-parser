use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ChangeRepresentation {
    pub actions: Vec<Action>,
    pub before: HashMap<String, serde_json::Value>,
    pub after: HashMap<String, serde_json::Value>,
    pub after_unknown: HashMap<String, serde_json::Value>,
    pub before_sensitive: HashMap<String, serde_json::Value>,
    pub after_sensitive: HashMap<String, serde_json::Value>,
    pub replace_paths: Option<Vec<Vec<String>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Action {
    NoOp,
    Create,
    Read,
    Update,
    Delete,
}
