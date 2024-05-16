use std::fmt;

use crate::{instructions::Instructions, InstructionAddress};

use super::{compiler::Instruction, symbols::Symbols};

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Code {
    pub symbols: Symbols,
    pub instructions: Instructions,
}

impl Code {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_address(&self) -> InstructionAddress {
        self.instructions.next_address()
    }

    pub fn push(&mut self, instruction: Instruction) -> InstructionAddress {
        let address = self.next_address();
        self.instructions.inner.push(instruction);
        address
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, instruction) in self.instructions.inner.iter().enumerate() {
            writeln!(f, "{i:4} {instruction}")?;
        }

        Ok(())
    }
}
