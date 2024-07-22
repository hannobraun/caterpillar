use std::collections::BTreeMap;

use crate::{
    operands::PopOperandError, Function, Instruction, InstructionAddr,
    Instructions, Operands, Value,
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
        instructions: &Instructions,
    ) -> Result<(), PushStackFrameError> {
        // We must create the new stack frame before we do tail call
        // optimization. Otherwise, we might drop the current frame, and if the
        // current frame is the top-level frame, then any potential arguments
        // for the new frame have nowhere to go.
        let mut new_frame = StackFrame::new(function);
        new_frame.take_arguments(self.frames.last_mut())?;

        if let Some(next_addr) = self.next_instruction_in_current_frame() {
            let next_instruction = instructions
                .get(&next_addr)
                .expect("Expected instruction referenced on stack to exist");

            // If the current function is finished, pop its stack frame before
            // pushing the next one. This is tail call optimization.
            if let Instruction::Return = next_instruction {
                self.pop_frame()
                    .expect("Currently executing; stack can't be empty");
            }
        }

        const RECURSION_LIMIT: usize = 8;
        if self.frames.len() >= RECURSION_LIMIT {
            return Err(PushStackFrameError::Overflow);
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
        Some(frame.take_next_instruction())
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

    fn take_arguments(
        &mut self,
        caller: Option<&mut StackFrame>,
    ) -> Result<(), PushStackFrameError> {
        if let Some(caller) = caller {
            for argument in self.function.arguments.iter().rev() {
                let value = caller.operands.pop_any()?;
                self.bindings.insert(argument.clone(), value);
            }
        } else {
            // If there's no caller, then there's no place to take arguments
            // from. Make sure that the function doesn't expect any.
            assert_eq!(self.function.arguments.len(), 0);
        }

        Ok(())
    }

    fn next_instruction(&self) -> InstructionAddr {
        self.function.first_instruction
    }

    fn take_next_instruction(&mut self) -> InstructionAddr {
        let next = self.function.first_instruction;
        self.function.first_instruction.increment();
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
