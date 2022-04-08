pub mod block_expressions_representation;
pub mod change_representation;
pub mod configuration_representation;
pub mod expression_representation;
pub mod plan_representation;
pub mod state_representation;
pub mod values_representation;

use plan_representation::PlanRepresentation;
use state_representation::StateRepresentation;

pub struct TerraformParser {}

impl TerraformParser {
    pub fn parse_state(input: &str) -> Result<StateRepresentation, serde_json::Error> {
        serde_json::from_str(&input)
    }

    pub fn parse_plan(input: &str) -> Result<PlanRepresentation, serde_json::Error> {
        serde_json::from_str(&input)
    }
}
