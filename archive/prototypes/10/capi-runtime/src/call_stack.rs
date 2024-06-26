use std::collections::BTreeMap;

use crate::{runtime::Function, InstructionAddress, Value};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallStack {
    frames: Vec<StackFrame>,
}

impl CallStack {
    pub fn new(next: Function) -> Self {
        let mut self_ = Self { frames: Vec::new() };
        self_
            .push(next)
            .expect("Expected recursion limit to be more than zero.");
        self_
    }

    pub fn next(&self) -> Option<InstructionAddress> {
        self.frames
            .last()
            .and_then(|frame| frame.function.front().copied())
    }

    pub fn contains(&self, address: InstructionAddress) -> bool {
        self.frames
            .iter()
            .any(|frame| frame.function.front() == Some(&address.next()))
    }

    pub fn push(
        &mut self,
        frame: impl Into<StackFrame>,
    ) -> Result<(), CallStackOverflow> {
        if self.frames.len() >= RECURSION_LIMIT {
            return Err(CallStackOverflow);
        }

        self.frames.push(frame.into());
        Ok(())
    }

    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }

    pub fn iter(&self) -> impl Iterator<Item = &InstructionAddress> {
        self.frames
            .iter()
            .filter_map(|frame| frame.function.front())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StackFrame {
    pub function: Function,
    pub bindings: Bindings,
}

impl From<Function> for StackFrame {
    fn from(function: Function) -> Self {
        Self {
            function,
            bindings: Bindings::new(),
        }
    }
}

pub type Bindings = BTreeMap<String, Value>;

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    thiserror::Error,
)]
#[error("Overflowed call stack")]
pub struct CallStackOverflow;

const RECURSION_LIMIT: usize = 8;
