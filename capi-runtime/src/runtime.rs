use std::collections::VecDeque;

use crate::InstructionAddress;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub instructions: VecDeque<InstructionAddress>,
}

impl Function {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            instructions: VecDeque::new(),
        }
    }
}
