use std::{fmt, str::FromStr};

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

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Instructions {
    pub inner: Vec<Instruction>,
}

impl Instructions {
    pub fn next_address(&self) -> InstructionAddress {
        InstructionAddress(self.inner.len().try_into().unwrap())
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct InstructionAddress(u32);

impl InstructionAddress {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn to_usize(self) -> usize {
        self.0.try_into().unwrap()
    }
}

impl FromStr for InstructionAddress {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let address = u32::from_str(s)?;
        Ok(Self(address))
    }
}
