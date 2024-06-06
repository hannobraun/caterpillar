use std::collections::BTreeMap;

use super::{DataStack, Function, InstructionAddress, Value};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallStack {
    frames: Vec<StackFrame>,
}

impl CallStack {
    pub fn new(next: Function) -> Self {
        let mut self_ = Self { frames: Vec::new() };
        self_
            .push(StackFrame::new(next))
            .expect("Expected recursion limit to be more than zero.");
        self_
    }

    pub fn next(&self) -> Option<InstructionAddress> {
        self.frames
            .last()
            .and_then(|frame| frame.function.instructions.front().copied())
    }

    pub fn top(&self) -> Option<&StackFrame> {
        self.frames.last()
    }

    pub fn top_mut(&mut self) -> Option<&mut StackFrame> {
        self.frames.last_mut()
    }

    pub fn contains(&self, address: InstructionAddress) -> bool {
        self.frames.iter().any(|frame| {
            frame.function.instructions.front() == Some(&address.next())
        })
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
            .filter_map(|frame| frame.function.instructions.front())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StackFrame {
    pub function: Function,
    pub data_stack: DataStack,
    pub bindings: Bindings,
}

impl StackFrame {
    pub fn new(function: Function) -> Self {
        Self {
            function,
            data_stack: DataStack::new(),
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
