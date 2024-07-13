use crate::instructions::{InstructionIndex, Instructions};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub first_instruction: Option<InstructionIndex>,
    pub num_instructions: u32,
    pub instructions: Instructions,
}
