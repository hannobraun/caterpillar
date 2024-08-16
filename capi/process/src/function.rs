use crate::{InstructionAddress, Pattern};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub branches: Vec<Branch>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Branch {
    pub parameters: Vec<Pattern>,
    pub start: InstructionAddress,
}
