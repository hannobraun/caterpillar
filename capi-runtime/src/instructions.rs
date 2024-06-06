use std::{fmt, str::FromStr};

use crate::Value;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Instructions {
    inner: InstructionsInner,
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

    pub fn get(&self, address: &InstructionAddress) -> &Instruction {
        let (stored_address, instruction) = &self.inner[address.to_usize()];
        assert_eq!(address, stored_address);
        instruction
    }
}

impl<'r> IntoIterator for &'r Instructions {
    type Item = <&'r InstructionsInner as IntoIterator>::Item;
    type IntoIter = <&'r InstructionsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

type InstructionsInner = Vec<(InstructionAddress, Instruction)>;

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

    pub fn next(mut self) -> Self {
        self.increment();
        self
    }

    pub fn to_usize(self) -> usize {
        self.0.try_into().unwrap()
    }
}

impl fmt::Display for InstructionAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
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
    BindingDefine { name: String },
    BindingEvaluate { name: String },
    CallBuiltin { name: String },
    CallFunction { name: String },
    Push { value: Value },
    ReturnIfNonZero,
    ReturnIfZero,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::BindingDefine { name } => write!(f, "bind `{name}`")?,
            Instruction::BindingEvaluate { name } => {
                write!(f, "eval binding `{name}`")?
            }
            Instruction::CallBuiltin { name } => write!(f, "builtin `{name}`")?,
            Instruction::CallFunction { name } => write!(f, "fn `{name}`")?,
            Instruction::Push { value } => write!(f, "push {value}")?,
            Instruction::ReturnIfNonZero => write!(f, "return if non-zero")?,
            Instruction::ReturnIfZero => write!(f, "return if zero")?,
        }

        Ok(())
    }
}
