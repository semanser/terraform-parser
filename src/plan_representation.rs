use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::change_representation::*;
use crate::configuration_representation::*;
use crate::state_representation::*;
use crate::values_representation::*;

#[derive(Serialize, Deserialize)]
pub struct PlanRepresentation {
    pub format_version: String,
    pub prior_state: StateRepresentation,
    pub configuration: ConfigurationRepresentation,
    pub planned_values: ValuesRepresentation,
    pub proposed_unknown: Option<ValuesRepresentation>,
    pub variables: HashMap<String, Variable>,
    pub resource_changes: Vec<ResourceChange>,
    pub output_changes: HashMap<String, OutputChange>,
}

#[derive(Serialize, Deserialize)]
pub struct Variable {
    pub value: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceChange {
    pub address: String,
    pub previous_address: Option<String>,
    pub module_address: Option<String>,
    pub mode: Mode,
    #[serde(rename = "type")]
    pub change_type: String,
    pub name: String,
    pub index: Option<serde_json::Value>,
    pub deposed: Option<String>,
    pub change: Option<ChangeRepresentation>,
    pub action_reason: Option<ActionReason>,
}

#[derive(Serialize, Deserialize)]
pub struct OutputChange {
    pub change: Option<ChangeRepresentation>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionReason {
    ReplaceBecauseTainted,
    ReplaceBecauseCannotUpdate,
    ReplaceByRequest,
    DeleteBecauseNoResourceConfig,
    DeleteBecauseNoModule,
    DeleteBecauseWrongRepetition,
    DeleteBecauseCountIndex,
    DeleteBecauseEachKey,
}
