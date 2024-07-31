use crate::instructions::InstructionAddress;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub arguments: Vec<String>,
    pub start: InstructionAddress,
}
