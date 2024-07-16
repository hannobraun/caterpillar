use std::collections::BTreeMap;

use crate::{
    operands::PopOperandError, Function, InstructionAddr, Operands, Value,
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

    pub fn next_instruction_in_current_frame(&self) -> Option<InstructionAddr> {
        Some(self.frames.last()?.next_instruction())
    }

    pub fn next_instruction_overall(&self) -> Option<InstructionAddr> {
        if let Some(frame) = self.frames.last() {
            return Some(frame.next_instruction());
        }

        None
    }

    pub fn is_next_instruction_in_any_frame(
        &self,
        instruction: &InstructionAddr,
    ) -> bool {
        let mut instruction = *instruction;
        instruction.increment();

        self.frames
            .iter()
            .any(|frame| frame.next_instruction() == instruction)
    }

    pub fn all_next_instructions_in_frames(
        &self,
    ) -> impl Iterator<Item = InstructionAddr> + '_ {
        self.frames.iter().map(|frame| frame.next_instruction())
    }

    pub fn push_frame(
        &mut self,
        function: Function,
    ) -> Result<(), PushStackFrameError> {
        const RECURSION_LIMIT: usize = 8;
        if self.frames.len() >= RECURSION_LIMIT {
            return Err(PushStackFrameError::Overflow);
        }

        let mut new_frame = StackFrame::new(function);

        if let Some(calling_frame) = self.frames.last_mut() {
            for argument in new_frame.function.arguments.iter().rev() {
                let value = calling_frame.operands.pop_any()?;
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

    pub fn pop_operand(&mut self) -> Result<Value, PopOperandError> {
        let frame = self.frames.last_mut().unwrap();
        frame.operands.pop_any()
    }

    pub fn take_next_instruction(&mut self) -> Option<InstructionAddr> {
        let frame = self.frames.last_mut()?;

        let instruction = frame.take_next_instruction();

        Some(instruction)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct StackFrame {
    pub function: Function,
    pub bindings: Bindings,
    pub operands: Operands,
}

impl StackFrame {
    fn new(function: Function) -> Self {
        Self {
            function,
            bindings: Bindings::default(),
            operands: Operands::default(),
        }
    }

    fn next_instruction(&self) -> InstructionAddr {
        self.function.instructions.first
    }

    fn take_next_instruction(&mut self) -> InstructionAddr {
        let next = self.function.instructions.first;
        self.function.instructions.first.increment();
        next
    }
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
    PopOperand(#[from] PopOperandError),

    #[error("Reached recursion limit")]
    Overflow,
}

#[derive(Debug)]
pub struct StackIsEmpty;
