use std::collections::BTreeMap;

use crate::{
    operands::MissingOperand,
    runtime::{Instruction, Value},
    Function, Location, Operands,
};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Stack {
    frames: Vec<StackFrame>,
}

impl Stack {
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

    pub fn push_frame(
        &mut self,
        function: Function,
    ) -> Result<(), PushStackFrameError> {
        const RECURSION_LIMIT: usize = 8;
        if self.frames.len() >= RECURSION_LIMIT {
            return Err(PushStackFrameError::Overflow);
        }

        let mut new_frame = StackFrame {
            function,
            bindings: Bindings::default(),
            operands: Operands::default(),
        };

        if let Some(calling_frame) = self.frames.last_mut() {
            for argument in new_frame.function.arguments.iter().rev() {
                let value = calling_frame.operands.pop()?;
                new_frame.bindings.insert(argument.clone(), value);
            }
        } else {
            // If there's no calling frame, then there's no place to take
            // arguments from. Make sure that the function doesn't expect any.
            assert_eq!(new_frame.function.arguments.len(), 0);
        }

        self.frames.push(new_frame);

        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<(), StackIsEmpty> {
        let Some(popped_frame) = self.frames.pop() else {
            return Err(StackIsEmpty);
        };

        if let Some(new_top_frame) = self.frames.last_mut() {
            for value in popped_frame.operands.values() {
                new_top_frame.operands.push(value);
            }
        }

        Ok(())
    }

    pub fn define_binding(&mut self, name: String, value: impl Into<Value>) {
        let frame = self.frames.last_mut().unwrap();
        frame.bindings.insert(name, value.into());
    }

    pub fn push_operand(&mut self, operand: impl Into<Value>) {
        let frame = self.frames.last_mut().unwrap();
        frame.operands.push(operand.into());
    }

    pub fn pop_operand(&mut self) -> Result<Value, MissingOperand> {
        let frame = self.frames.last_mut().unwrap();
        frame.operands.pop()
    }

    pub fn consume_next_instruction(&mut self) -> Option<Instruction> {
        loop {
            let frame = self.frames.last_mut()?;

            let Some(instruction) = frame.function.consume_next_instruction()
            else {
                self.pop_frame()
                    .expect("Just accessed frame; must be able to pop it");
                continue;
            };

            return Some(instruction);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct StackFrame {
    pub function: Function,
    pub bindings: Bindings,
    pub operands: Operands,
}

pub type Bindings = BTreeMap<String, Value>;

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum PushStackFrameError {
    #[error(transparent)]
    MissingOperand(#[from] MissingOperand),

    #[error("Reached recursion limit")]
    Overflow,
}

#[derive(Debug)]
pub struct StackIsEmpty;
