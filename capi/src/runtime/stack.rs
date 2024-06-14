use std::collections::BTreeMap;

use super::{DataStack, Function, Location, Value};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stack {
    frames: Vec<StackFrame>,
}

impl Stack {
    pub fn new(next: Function) -> Self {
        let mut self_ = Self { frames: Vec::new() };
        self_
            .push(StackFrame::new(next))
            .expect("Expected recursion limit to be more than zero.");
        self_
    }

    pub fn next(&self) -> Option<Location> {
        self.frames
            .last()
            .and_then(|frame| frame.function.next_instruction())
            .map(|(location, _instruction)| location)
    }

    pub fn top_frame(&self) -> Option<&StackFrame> {
        self.frames.last()
    }

    pub fn top_frame_mut(&mut self) -> Option<&mut StackFrame> {
        self.frames.last_mut()
    }

    pub fn contains(&self, location: &Location) -> bool {
        self.frames.iter().any(|frame| {
            frame
                .function
                .next_instruction()
                .map(|(location, _instruction)| location)
                == Some(location.clone().next())
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

    pub fn iter(&self) -> impl Iterator<Item = Location> + '_ {
        self.frames
            .iter()
            .filter_map(|frame| frame.function.next_instruction())
            .map(|(location, _instruction)| location)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackFrame {
    pub function: Function,
    pub data: DataStack,
    pub bindings: Bindings,
}

impl StackFrame {
    pub fn new(function: Function) -> Self {
        Self {
            function,
            data: DataStack::new(),
            bindings: Bindings::new(),
        }
    }
}

pub type Bindings = BTreeMap<String, Value>;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("Overflowed call stack")]
pub struct CallStackOverflow;

const RECURSION_LIMIT: usize = 8;
