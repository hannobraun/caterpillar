use crate::instructions::{InstructionIndex, Instructions};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub first_instruction: Option<InstructionSlice>,
    pub instructions: Instructions,
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct InstructionSlice {
    pub first: InstructionIndex,
    pub len: u32,
}
