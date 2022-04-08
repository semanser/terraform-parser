use crate::block_expressions_representation::*;
use crate::expression_representation::*;
use crate::values_representation::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ConfigurationRepresentation {
    pub provider_configs: Option<HashMap<String, ProviderConfig>>,
    pub root_module: ModuleConfiguration,
}

#[derive(Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub alias: String,
    pub module_address: String,
    pub expressions: BlockExpressionRepresentation,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleConfiguration {
    pub outputs: HashMap<String, Output>,
    pub resources: Vec<Resource>,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub expression: serde_json::Value,
    pub sensetive: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub address: String,
    pub mode: Mode,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub name: String,
    pub provider_config_key: String,
    pub provisioners: Option<Vec<Provisioner>>,
    pub expressions: BlockExpressionRepresentation,
    pub schema_version: i8,
    pub count_expression: Option<ExpressionRepresentation>,
    pub for_each_expression: Option<ExpressionRepresentation>,
    pub module_calls: Option<HashMap<String, ModuleCall>>,
}

#[derive(Serialize, Deserialize)]
pub struct Provisioner {
    #[serde(rename = "type")]
    pub provisioner_type: String,
    pub expression: BlockExpressionRepresentation,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleCall {
    pub resolved_source: String,
    pub expressions: BlockExpressionRepresentation,
    pub count_expression: Option<ExpressionRepresentation>,
    pub for_each_expression: Option<ExpressionRepresentation>,
    pub module: ModuleConfiguration,
}
