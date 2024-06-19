use std::collections::BTreeMap;

use crate::runtime::{Function, Location, Operands, Value};

use super::Event;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State {
    pub frames: Vec<StackFrame>,
}

impl State {
    pub fn num_frames(&self) -> usize {
        self.frames.len()
    }

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

    pub fn next_instruction_overall(&self) -> Option<Location> {
        for frame in self.frames.iter().rev() {
            if let Some((location, _)) = frame.function.next_instruction() {
                return Some(location);
            }
        }

        None
    }

    pub fn is_next_instruction_in_any_frame(
        &self,
        location: &Location,
    ) -> bool {
        self.frames.iter().any(|frame| {
            frame
                .function
                .next_instruction()
                .map(|(location, _instruction)| location)
                == Some(location.clone().next())
        })
    }

    pub fn all_next_instructions_in_frames(
        &self,
    ) -> impl Iterator<Item = Location> + '_ {
        self.frames
            .iter()
            .filter_map(|frame| frame.function.next_instruction())
            .map(|(location, _instruction)| location)
    }

    pub fn evolve(&mut self, event: Event) {
        match event {
            Event::PushFrame { function } => {
                self.frames.push(StackFrame {
                    function,
                    bindings: Bindings::default(),
                    operands: Operands::default(),
                });
            }
            Event::PopFrame => {
                self.frames.pop();
            }
            Event::DefineBinding { name, value } => {
                let frame = self.frames.last_mut().expect(
                    "`Event::DefineBinding` implies existence of stack frame",
                );
                frame.bindings.insert(name, value);
            }
            Event::PushOperand { operand: value } => {
                let frame = self.frames.last_mut().expect(
                    "`Event::PushOperand` implies existence of stack frame",
                );
                frame.operands.push(value);
            }
            Event::PopOperand { operand } => {
                let frame = self.frames.last_mut().expect(
                    "`Event::PopOperand` implies existence of stack frame",
                );
                *operand = frame.operands.pop();
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackFrame {
    pub function: Function,
    pub bindings: Bindings,
    pub operands: Operands,
}

pub type Bindings = BTreeMap<String, Value>;
