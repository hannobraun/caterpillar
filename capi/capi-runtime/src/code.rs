use std::fmt;

use crate::{
    instructions::{Instruction, Instructions},
    InstructionAddress,
};

use super::symbols::Symbols;

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
        self.instructions.push(instruction)
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (address, instruction) in &self.instructions {
            write!(f, "{address:4} {instruction}")?;

            if let Some(name) = self.symbols.resolve_address(address) {
                writeln!(f, " ({name})")?;
            } else {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
