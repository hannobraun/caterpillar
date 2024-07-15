use std::{collections::VecDeque, fmt};

use crate::Value;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Instructions {
    inner: InstructionsInner,
}

impl Instructions {
    pub fn push(&mut self, instruction: Instruction) -> InstructionAddr {
        let addr = InstructionAddr {
            index: self.inner.len().try_into().unwrap(),
        };
        self.inner.push_back((addr, instruction));
        addr
    }

    pub fn get(&self, addr: &InstructionAddr) -> Option<&Instruction> {
        let (stored_addr, instruction) = self.inner.get(addr.to_usize())?;
        assert_eq!(addr, stored_addr);
        Some(instruction)
    }
}

impl<'r> IntoIterator for &'r Instructions {
    type Item = <&'r InstructionsInner as IntoIterator>::Item;
    type IntoIter = <&'r InstructionsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

type InstructionsInner = VecDeque<(InstructionAddr, Instruction)>;

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
pub struct InstructionAddr {
    pub index: u32,
}

impl InstructionAddr {
    pub fn increment(&mut self) {
        self.index += 1;
    }

    fn to_usize(self) -> usize {
        self.index
            .try_into()
            .expect("Expected `usize` to cover full range of `u32`")
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
