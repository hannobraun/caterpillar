use std::collections::BTreeMap;

use super::{Function, Instruction, Location, Operands, StackUnderflow, Value};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stack {
    frames: Vec<StackFrame>,
}

impl Stack {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn next_instruction(&self) -> Option<Location> {
        for frame in self.frames.iter().rev() {
            if let Some((location, _)) = frame.function.next_instruction() {
                return Some(location);
            }
        }

        None
    }

    pub fn top_frame(&self) -> Option<&StackFrame> {
        self.frames.last()
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

    pub fn push_frame(&mut self, function: Function) -> Result<(), PushError> {
        const RECURSION_LIMIT: usize = 8;
        if self.frames.len() >= RECURSION_LIMIT {
            return Err(PushError::Overflow);
        }

        let mut bindings = Bindings::new();

        if let Some(calling_frame) = self.frames.last_mut() {
            for argument in function.arguments.iter().rev() {
                let value = calling_frame.data.pop()?;
                bindings.insert(argument.clone(), value);
            }
        } else {
            // If there's no calling frame, then there's no place to take
            // arguments from. Make sure that the function doesn't expect any.
            assert_eq!(function.arguments.len(), 0);
        }

        self.frames.push(StackFrame {
            function,
            bindings,
            data: Operands::new(),
        });

        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<StackFrame, StackIsEmpty> {
        let old_top = self.frames.pop().ok_or(StackIsEmpty)?;

        if let Some(new_top) = self.frames.last_mut() {
            for value in old_top.data.values() {
                new_top.data.push(value);
            }
        }

        Ok(old_top)
    }

    pub fn push_value(&mut self, value: Value) {
        self.frames.last_mut().unwrap().data.push(value);
    }

    pub fn consume_next_instruction<R>(
        &mut self,
        f: impl FnOnce(Location, Instruction, &mut Operands, &mut Bindings) -> R,
    ) -> Option<R> {
        loop {
            let frame = self.frames.last_mut()?;

            let Some((location, instruction)) =
                frame.function.consume_next_instruction()
            else {
                self.pop_frame()
                    .expect("Just accessed a frame; expecting to pop it");
                continue;
            };

            let result =
                f(location, instruction, &mut frame.data, &mut frame.bindings);

            // Don't put the stack frame back, if it is empty. This is tail call
            // optimization.
            //
            // This will lead to trouble, if the last instruction in the
            // function (the one we just executed) is an explicit return
            // instruction. Those pop *another* stack frame, which is one too
            // many.
            //
            // I've decided not to address that, for the moment:
            //
            // 1. That is a weird pattern anyway, and doesn't really make sense
            //    to write.
            // 2. Explicit return instructions are a stopgap anyway, until we
            //    have more advanced control flow.
            let frame_is_empty = frame.function.next_instruction().is_none();
            if frame_is_empty {
                self.pop_frame()
                    .expect("Just accessed a frame; expecting to pop it");
            }

            return Some(result);
        }
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
    pub bindings: Bindings,
    pub data: Operands,
}

pub type Bindings = BTreeMap<String, Value>;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PushError {
    #[error("Reached recursion limit")]
    Overflow,

    #[error("Expected function arguments on stack")]
    MissingArgument(#[from] StackUnderflow),
}

#[derive(Debug)]
pub struct StackIsEmpty;
