use crate::runtime::{
    Function, Instruction, Location, MissingOperand, Operands, Value,
};

use super::{state::StackFrame, Bindings, State};

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

    pub fn next_instruction_in_current_frame(&self) -> Option<Location> {
        self.state.next_instruction_in_current_frame()
    }

    pub fn next_instruction_overall(&self) -> Option<Location> {
        self.state.next_instruction_overall()
    }

    pub fn bindings(&self) -> Option<&Bindings> {
        self.state.bindings()
    }

    pub fn operands(&self) -> Option<&Operands> {
        self.state.operands()
    }

    pub fn is_next_instruction_in_any_frame(
        &self,
        location: &Location,
    ) -> bool {
        self.state.is_next_instruction_in_any_frame(location)
    }

    pub fn push_frame(&mut self, function: Function) -> Result<(), PushError> {
        const RECURSION_LIMIT: usize = 8;
        if self.state.frames.len() >= RECURSION_LIMIT {
            return Err(PushError::Overflow);
        }

        let mut bindings = Bindings::new();

        if let Some(calling_frame) = self.state.frames.last_mut() {
            for argument in function.arguments.iter().rev() {
                let value = calling_frame.operands.pop()?;
                bindings.insert(argument.clone(), value);
            }
        } else {
            // If there's no calling frame, then there's no place to take
            // arguments from. Make sure that the function doesn't expect any.
            assert_eq!(function.arguments.len(), 0);
        }

        self.state.frames.push(StackFrame {
            function,
            bindings,
            operands: Operands::new(),
        });

        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<StackFrame, StackIsEmpty> {
        let old_top = self.state.frames.pop().ok_or(StackIsEmpty)?;
        self.return_values(&old_top);
        Ok(old_top)
    }

    pub fn define_binding(&mut self, name: String, value: impl Into<Value>) {
        self.state
            .frames
            .last_mut()
            .unwrap()
            .bindings
            .insert(name, value.into());
    }

    pub fn push_operand(&mut self, operand: impl Into<Value>) {
        self.state.frames.last_mut().unwrap().operands.push(operand);
    }

    pub fn pop_operand(&mut self) -> Result<Value, MissingOperand> {
        self.state.frames.last_mut().unwrap().operands.pop()
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

    pub fn iter(&self) -> impl Iterator<Item = Location> + '_ {
        self.state
            .frames
            .iter()
            .filter_map(|frame| frame.function.next_instruction())
            .map(|(location, _instruction)| location)
    }

    fn return_values(&mut self, frame: &StackFrame) {
        if let Some(new_top) = self.state.frames.last_mut() {
            for value in frame.operands.values() {
                new_top.operands.push(value);
            }
        }
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
