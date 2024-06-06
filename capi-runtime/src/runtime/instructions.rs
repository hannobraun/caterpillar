use std::fmt;

use super::Value;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Instructions {
    inner: InstructionsInner,
}

impl Instructions {
    pub fn next_location(&self) -> Location {
        Location {
            index: self.inner.len().try_into().unwrap(),
        }
    }

    pub fn push(&mut self, instruction: Instruction) -> Location {
        let location = self.next_location();
        self.inner.push((location, instruction));
        location
    }

    pub fn get(&self, location: &Location) -> &Instruction {
        let (stored_location, instruction) = &self.inner[location.to_index()];
        assert_eq!(location, stored_location);
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

type InstructionsInner = Vec<(Location, Instruction)>;

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
pub struct Location {
    index: u32,
}

impl Location {
    pub fn next(mut self) -> Self {
        self.increment();
        self
    }

    fn increment(&mut self) {
        self.index += 1;
    }

    fn to_index(self) -> usize {
        self.index.try_into().unwrap()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.index.fmt(f)
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
