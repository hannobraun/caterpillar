use std::{fmt, str::FromStr};

use crate::Value;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Instructions {
    pub inner: Vec<(InstructionAddress, Instruction)>,
}

impl Instructions {
    pub fn next_address(&self) -> InstructionAddress {
        InstructionAddress(self.inner.len().try_into().unwrap())
    }

    pub fn push(&mut self, instruction: Instruction) -> InstructionAddress {
        let address = self.next_address();
        self.inner.push((address, instruction));
        address
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

impl fmt::Display for InstructionAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InstructionAddress {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let address = u32::from_str(s)?;
        Ok(Self(address))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Instruction {
    CallBuiltin { name: String },
    CallFunction { name: String },
    Push { value: Value },
    Return,
    ReturnIfNonZero,
    ReturnIfZero,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::CallBuiltin { name } => write!(f, "builtin `{name}`"),
            Instruction::CallFunction { name } => write!(f, "fn `{name}`"),
            Instruction::Push { value } => write!(f, "push {value}"),
            Instruction::Return => write!(f, "return"),
            Instruction::ReturnIfNonZero => write!(f, "return if non-zero"),
            Instruction::ReturnIfZero => write!(f, "return if zero"),
        }
    }
}
