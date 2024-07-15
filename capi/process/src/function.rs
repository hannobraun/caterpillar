use crate::instructions::InstructionAddr;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub instructions: FunctionInstructions,
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct FunctionInstructions {
    pub first: Option<InstructionAddr>,
    pub count: u32,
}
