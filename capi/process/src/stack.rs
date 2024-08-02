use std::collections::BTreeMap;

use crate::{
    operands::PopOperandError, Function, Instruction, InstructionAddress,
    Instructions, Operands, Value,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Stack {
    inner: Vec<StackElement>,
    next_instruction: InstructionAddress,

    /// # Special heap for closures
    ///
    /// ## Implementation Note
    ///
    /// This doesn't belong here. It just was a convenient place to put it, as
    /// all code that needs to deal with closures has access to `Stack`.
    ///
    /// The eventual plan is to put closures on the regular stack, but that is
    /// likely to be impractical while the language is untyped.
    pub closures: BTreeMap<u32, (InstructionAddress, BTreeMap<String, Value>)>,
    pub next_closure: u32,
}

impl Stack {
    pub fn new() -> Self {
        let next_instruction = InstructionAddress { index: 0 };
        let inner =
            vec![StackElement::Frame(StackFrame::new(next_instruction))];

        Self {
            inner,
            next_instruction,
            closures: BTreeMap::new(),
            next_closure: 0,
        }
    }

    pub fn next_instruction(&self) -> InstructionAddress {
        self.next_instruction
    }

    pub fn bindings(&self) -> Option<&Bindings> {
        self.inner.last().and_then(|frame| match frame {
            StackElement::ReturnAddress(_) => None,
            StackElement::Frame(frame) => Some(&frame.bindings),
        })
    }

    pub fn operands(&self) -> Option<&Operands> {
        self.inner.last().and_then(|frame| match frame {
            StackElement::ReturnAddress(_) => None,
            StackElement::Frame(frame) => Some(&frame.operands),
        })
    }

    pub fn all_next_instructions_in_frames(
        &self,
    ) -> impl Iterator<Item = InstructionAddress> + '_ {
        self.inner
            .iter()
            .filter_map(|frame| match frame {
                StackElement::ReturnAddress(address) => Some(*address),
                StackElement::Frame(_) => None,
            })
            .chain([self.next_instruction])
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
        let mut new_frame = StackFrame::new(function.start);

        // Move arguments into the new frame.
        if let Some(StackElement::Frame(caller)) = self.inner.last_mut() {
            for argument in function.arguments.iter().rev() {
                let value = caller.operands.pop_any()?;
                new_frame.bindings.insert(argument.clone(), value);
            }
        } else {
            assert_eq!(
                function.arguments.len(),
                0,
                "Function has no caller, which means there is no stack frame \
                that the function could take its arguments from. Yet, it has \
                arguments, which can't work.",
            );
        }

        let next_addr = self.next_instruction();
        let next_instruction = instructions
            .get(&next_addr)
            .expect("Expected instruction referenced on stack to exist");

        // If the current function is finished, pop its stack frame before
        // pushing the next one. This is tail call optimization.
        if let Instruction::Return = next_instruction {
            self.pop_frame()
                .expect("Currently executing; stack can't be empty");
        }

        const RECURSION_LIMIT: usize = 16;
        if self.inner.len() >= RECURSION_LIMIT {
            return Err(PushStackFrameError::Overflow);
        }

        if !self.inner.is_empty() {
            self.inner
                .push(StackElement::ReturnAddress(self.next_instruction));
        }

        self.next_instruction = new_frame.next_instruction;
        self.inner.push(StackElement::Frame(new_frame));

        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<(), StackIsEmpty> {
        let Some(StackElement::Frame(popped_frame)) = self.inner.pop() else {
            return Err(StackIsEmpty);
        };
        if let Some(StackElement::ReturnAddress(address)) = self.inner.pop() {
            self.next_instruction = address;
        }

        if let Some(StackElement::Frame(new_top_frame)) = self.inner.last_mut()
        {
            for value in popped_frame.operands.values() {
                new_top_frame.operands.push(value);
            }
        }

        Ok(())
    }

    pub fn define_binding(&mut self, name: String, value: impl Into<Value>) {
        if let Some(StackElement::Frame(frame)) = self.inner.last_mut() {
            frame.bindings.insert(name, value.into());
        } else {
            panic!("Expected stack frame to exist.");
        }
    }

    pub fn push_operand(&mut self, operand: impl Into<Value>) {
        if let Some(StackElement::Frame(frame)) = self.inner.last_mut() {
            frame.operands.push(operand.into());
        } else {
            panic!("Expected stack frame to exist.");
        }
    }

    pub fn pop_operand(&mut self) -> Result<Value, PopOperandError> {
        if let Some(StackElement::Frame(frame)) = self.inner.last_mut() {
            frame.operands.pop_any()
        } else {
            panic!("Expected stack frame to exist.");
        }
    }

    pub fn take_next_instruction(&mut self) -> Option<InstructionAddress> {
        let StackElement::Frame(frame) = self.inner.last_mut()? else {
            return None;
        };

        let next_instruction = frame.take_next_instruction();
        self.next_instruction = frame.next_instruction;

        Some(next_instruction)
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
enum StackElement {
    ReturnAddress(InstructionAddress),
    Frame(StackFrame),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct StackFrame {
    pub next_instruction: InstructionAddress,
    pub bindings: Bindings,
    pub operands: Operands,
}

impl StackFrame {
    fn new(next_instruction: InstructionAddress) -> Self {
        Self {
            next_instruction,
            bindings: Bindings::default(),
            operands: Operands::default(),
        }
    }

    fn take_next_instruction(&mut self) -> InstructionAddress {
        let next = self.next_instruction;
        self.next_instruction.increment();
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
