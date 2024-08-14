use capi_process::Value;

use super::Expression;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub branches: Vec<Branch>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Branch {
    pub name: String,
    pub parameters: Vec<Pattern>,
    pub body: Vec<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Pattern {
    Identifier { name: String },
    Literal { value: Value },
}
