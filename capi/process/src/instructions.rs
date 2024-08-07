use std::{
    collections::{BTreeSet, VecDeque},
    fmt,
};

use crate::Value;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Instructions {
    inner: InstructionsInner,
}

impl Instructions {
    pub fn push(&mut self, instruction: Instruction) -> InstructionAddress {
        let address = InstructionAddress {
            index: self.inner.len().try_into().unwrap(),
        };
        self.inner.push_back((address, instruction));
        address
    }

    pub fn get(&self, address: &InstructionAddress) -> Option<&Instruction> {
        let (stored_address, instruction) =
            self.inner.get(address.to_usize())?;
        assert_eq!(address, stored_address);
        Some(instruction)
    }

    pub fn replace(
        &mut self,
        address: InstructionAddress,
        instruction: Instruction,
    ) {
        let (stored_address, stored_instruction) =
            self.inner.get_mut(address.to_usize()).unwrap();
        assert_eq!(address, *stored_address);
        *stored_instruction = instruction;
    }
}

impl<'r> IntoIterator for &'r Instructions {
    type Item = <&'r InstructionsInner as IntoIterator>::Item;
    type IntoIter = <&'r InstructionsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

type InstructionsInner = VecDeque<(InstructionAddress, Instruction)>;

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
pub struct InstructionAddress {
    pub index: u32,
}

impl InstructionAddress {
    pub fn increment(&mut self) {
        self.index += 1;
    }

    fn to_usize(self) -> usize {
        self.index
            .try_into()
            .expect("Expected `usize` to cover full range of `u32`")
    }
}

impl fmt::Display for InstructionAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.index)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Instruction {
    BindingEvaluate {
        name: String,
    },
    BindingsDefine {
        names: Vec<String>,
    },
    CallBuiltin {
        name: String,
    },
    CallCluster {
        cluster: Vec<(Vec<Pattern>, InstructionAddress)>,
        is_tail_call: bool,
    },
    MakeClosure {
        address: InstructionAddress,
        environment: BTreeSet<String>,
    },
    Push {
        value: Value,
    },
    Return,
    ReturnIfNonZero,
    ReturnIfZero,
    Panic,
}

/// # A pattern in a function argument
///
/// ## Implementation Note
///
/// This duplicates the type of the same name in `capi-compiler`. This is
/// deliberate, as a shared type would have to live here (as `capi-compiler`
/// depends on this crate), but it doesn't belong here. The need for this type
/// is temporary, while so much of pattern matching is still figured out at
/// runtime.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Pattern {
    Identifier { name: String },
    Literal { value: Value },
}
