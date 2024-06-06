use std::collections::BTreeMap;

use crate::runtime::{self, Instruction, InstructionAddress, Instructions};

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Code {
    pub instructions: Instructions,
    pub functions: BTreeMap<String, runtime::Function>,
}

impl Code {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, instruction: Instruction) -> InstructionAddress {
        self.instructions.push(instruction)
    }
}
