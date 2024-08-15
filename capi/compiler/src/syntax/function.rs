use std::collections::BTreeSet;

use capi_process::Value;

use super::Expression;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: Option<String>,

    pub branches: Vec<Branch>,

    /// The environment of the function
    ///
    /// These are the values that the function captured from parent scopes.
    ///
    /// The environment is empty on construction, until it is filled in during
    /// the resolve pass.
    pub environment: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Branch {
    pub parameters: Vec<Pattern>,
    pub body: Vec<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Pattern {
    Identifier { name: String },
    Literal { value: Value },
}
