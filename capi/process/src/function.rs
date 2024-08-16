use crate::{InstructionAddress, Pattern};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub branches: Vec<(Vec<Pattern>, InstructionAddress)>,
}
