use crate::ExpressionNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionExpression {
    pub name: String,
    pub args: ExpressionNode,
}