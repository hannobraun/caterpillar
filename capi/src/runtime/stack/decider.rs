use crate::runtime::{Function, Instruction, MissingOperand, Value};

use super::{state::StackFrame, Bindings, Event, State};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stack {
    state: State,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            state: State::default(),
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn push_frame(&mut self, function: Function) -> Result<(), PushError> {
        const RECURSION_LIMIT: usize = 8;
        if self.state.num_frames() >= RECURSION_LIMIT {
            return Err(PushError::Overflow);
        }

        if self.state.num_frames() == 0 {
            // If there's no calling frame, then there's no place to take
            // arguments from. Make sure that the function doesn't expect any.
            assert_eq!(function.arguments.len(), 0);
        }

        let mut arguments = Bindings::new();

        for argument in function.arguments.iter().rev() {
            let value = self.pop_operand()?;
            arguments.insert(argument.clone(), value);
        }

        self.emit_event(Event::PushFrame { function });

        for (name, value) in arguments {
            self.define_binding(name, value);
        }

        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<StackFrame, StackIsEmpty> {
        let old_top = self.state.frames.pop().ok_or(StackIsEmpty)?;
        self.return_values(&old_top);
        Ok(old_top)
    }

    pub fn define_binding(&mut self, name: String, value: impl Into<Value>) {
        let value = value.into();
        self.state
            .frames
            .last_mut()
            .unwrap()
            .bindings
            .insert(name, value);
    }

    pub fn push_operand(&mut self, operand: impl Into<Value>) {
        self.state.frames.last_mut().unwrap().operands.push(operand);
    }

    pub fn pop_operand(&mut self) -> Result<Value, MissingOperand> {
        let mut value = Err(MissingOperand);
        self.emit_event(Event::PopOperand { value: &mut value });
        value
    }

    pub fn consume_next_instruction(&mut self) -> Option<Instruction> {
        loop {
            let frame = self.state.frames.last_mut()?;

            let Some(instruction) = frame.function.consume_next_instruction()
            else {
                self.pop_frame()
                    .expect("Just accessed frame; must be able to pop it");
                continue;
            };

            return Some(instruction);
        }
    }

    fn return_values(&mut self, frame: &StackFrame) {
        if let Some(new_top) = self.state.frames.last_mut() {
            for value in frame.operands.values() {
                new_top.operands.push(value);
            }
        }
    }

    fn emit_event(&mut self, event: Event) {
        self.state.evolve(event);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PushError {
    #[error("Reached recursion limit")]
    Overflow,

    #[error("Expected function arguments on stack")]
    MissingArgument(#[from] MissingOperand),
}

#[derive(Debug)]
pub struct StackIsEmpty;
