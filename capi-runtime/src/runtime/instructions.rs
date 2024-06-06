use std::fmt;

use super::Value;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Instructions {
    inner: InstructionsInner,
}

impl Instructions {
    pub fn next_location(&self) -> InstructionAddress {
        InstructionAddress(self.inner.len().try_into().unwrap())
    }

    pub fn push(&mut self, instruction: Instruction) -> InstructionAddress {
        let address = self.next_location();
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
    pub fn next(mut self) -> Self {
        self.increment();
        self
    }

    fn increment(&mut self) {
        self.0 += 1;
    }

    fn to_usize(self) -> usize {
        self.0.try_into().unwrap()
    }
}

impl fmt::Display for InstructionAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Instruction {
    BindingEvaluate { name: String },
    BindingsDefine { names: Vec<String> },
    CallBuiltin { name: String },
    CallFunction { name: String },
    Push { value: Value },
    ReturnIfNonZero,
    ReturnIfZero,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::BindingEvaluate { name } => {
                write!(f, "eval binding `{name}`")?;
            }
            Instruction::BindingsDefine { names } => {
                write!(f, "bind")?;
                for name in names {
                    write!(f, " `{name}`")?;
                }
            }
            Instruction::CallBuiltin { name } => {
                write!(f, "builtin `{name}`")?;
            }
            Instruction::CallFunction { name } => {
                write!(f, "fn `{name}`")?;
            }
            Instruction::Push { value } => {
                write!(f, "push {value}")?;
            }
            Instruction::ReturnIfNonZero => {
                write!(f, "return if non-zero")?;
            }
            Instruction::ReturnIfZero => {
                write!(f, "return if zero")?;
            }
        }

        Ok(())
    }
}
