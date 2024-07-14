use crate::instructions::InstructionIndex;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub instructions: Option<InstructionSlice>,
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct InstructionSlice {
    pub first: InstructionIndex,
    pub len: u32,
}
