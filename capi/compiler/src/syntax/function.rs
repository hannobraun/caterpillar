use capi_process::Value;

use super::Expression;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,

    /// The index of this function within its group
    ///
    /// Functions with the same name are grouped, and each function within that
    /// group is identified by its index. The purpose of this is to support
    /// pattern matching in function definitions.
    ///
    /// This starts out as `None`, and is filled in by a compiler pass that
    /// groups the functions.
    pub group_index: Option<u32>,

    pub arguments: Vec<Pattern>,
    pub body: Vec<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Pattern {
    Identifier { name: String },
    Literal { value: Value },
}
