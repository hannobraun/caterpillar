use std::collections::BTreeMap;

use crate::{InstructionAddress, Pattern, Value};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub branches: Vec<Branch>,
    pub environment: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Branch {
    pub parameters: Vec<Pattern>,
    pub start: InstructionAddress,
}
