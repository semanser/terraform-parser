use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExpressionRepresentation {
    pub count_expression: Option<String>,
    pub references: Vec<String>,
}
