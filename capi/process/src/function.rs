use crate::instructions::InstructionAddr;

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
    pub first: InstructionAddr,
    pub len: u32,
}
