use std::{collections::BTreeMap, fmt};

use crate::{
    instructions::{Instruction, Instructions}, runtime, InstructionAddress
};

use super::symbols::Symbols;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Code {
    pub symbols: Symbols,
    pub instructions: Instructions,
    pub functions: BTreeMap<String, runtime::Function>,
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
            if let Some(name) = self.symbols.resolve_address(address) {
                writeln!(f)?;
                writeln!(f, "{name}:")?;
            }

            writeln!(f, "{address:4} {instruction}")?;
        }

        Ok(())
    }
}
