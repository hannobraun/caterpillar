use capi_process::Value;

use super::Expression;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub group_index: Option<u32>,
    pub arguments: Vec<Pattern>,
    pub body: Vec<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Pattern {
    Identifier { name: String },
    Literal { value: Value },
}
