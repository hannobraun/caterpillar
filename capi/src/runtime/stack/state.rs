use std::collections::BTreeMap;

use crate::runtime::{Function, Location, Operands, Value};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State {
    pub frames: Vec<StackFrame>,
}

impl State {
    pub fn bindings(&self) -> Option<&Bindings> {
        self.frames.last().map(|frame| &frame.bindings)
    }

    pub fn operands(&self) -> Option<&Operands> {
        self.frames.last().map(|frame| &frame.operands)
    }

    pub fn next_instruction_in_current_frame(&self) -> Option<Location> {
        self.frames
            .last()?
            .function
            .next_instruction()
            .map(|(location, _)| location)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackFrame {
    pub function: Function,
    pub bindings: Bindings,
    pub operands: Operands,
}

pub type Bindings = BTreeMap<String, Value>;
