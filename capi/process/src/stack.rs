use std::collections::BTreeMap;

use crate::{
    operands::PopOperandError, Function, Instruction, InstructionAddress,
    Instructions, Value,
};

/// # Caterpillar's stack, supposedly
///
/// ## Implementation Note
///
/// This is more and more turning into a general "evaluator state" struct, of
/// which the actual stack is just one part. For now, I'm going to accept this.
///
/// This code is under heavy construction, and I think the best time to clean up
/// is when the new structure has fallen into place. Trying to separate it into
/// sensible components before that, will only cause unnecessary churn.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Stack {
    inner: Vec<StackElement>,
    legacy_stack: Vec<()>,
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
        Self {
            inner: Vec::new(),
            legacy_stack: vec![()],
            next_instruction: InstructionAddress { index: 0 },
            closures: BTreeMap::new(),
            next_closure: 0,
        }
    }

    pub fn next_instruction(&self) -> InstructionAddress {
        self.next_instruction
    }

    pub fn bindings(&self) -> Option<&Bindings> {
        self.inner.iter().rev().find_map(|element| match element {
            StackElement::Bindings(bindings) => Some(bindings),
            _ => None,
        })
    }

    pub fn bindings_mut(&mut self) -> Option<&mut Bindings> {
        self.inner
            .iter_mut()
            .rev()
            .find_map(|element| match element {
                StackElement::Bindings(bindings) => Some(bindings),
                _ => None,
            })
    }

    pub fn operands_in_current_stack_frame(
        &self,
    ) -> impl Iterator<Item = &Value> + '_ {
        self.inner
            .iter()
            .rev()
            .take_while(|element| {
                !matches!(element, StackElement::ReturnAddress(_))
            })
            .filter_map(|element| match element {
                StackElement::Operand(value) => Some(value),
                _ => None,
            })
    }

    pub fn active_instructions(
        &self,
    ) -> impl Iterator<Item = InstructionAddress> + '_ {
        self.inner
            .iter()
            .filter_map(|frame| match frame {
                StackElement::Bindings(_) => None,
                StackElement::Operand(_) => None,
                StackElement::ReturnAddress(address) => Some(*address),
            })
            .chain([self.next_instruction])
    }

    pub fn push_frame(
        &mut self,
        function: Function,
        instructions: &Instructions,
    ) -> Result<(), PushStackFrameError> {
        let arguments = function
            .arguments
            .into_iter()
            .rev()
            .map(|name| {
                let value = self.pop_operand()?;
                Ok((name, value))
            })
            .collect::<Result<Vec<_>, PushStackFrameError>>()?;
        let is_tail_call = {
            let next_instruction = instructions
                .get(&self.next_instruction)
                .expect("Expected instruction referenced on stack to exist");

            *next_instruction == Instruction::Return
        };

        // If the current function is finished, pop its stack frame before
        // pushing the next one. This is tail call optimization.
        if is_tail_call {
            self.pop_frame()
                .expect("Currently executing; stack can't be empty");
        }

        const RECURSION_LIMIT: usize = 16;
        if self.inner.len() >= RECURSION_LIMIT {
            // Applied to the new stack, this is no longer the recursion limit.
            // But it limits the size of the stack, which serves its purpose for
            // now.
            //
            // Once the legacy stack is gone, this can be cleaned up.
            return Err(PushStackFrameError::Overflow);
        }

        if !self.legacy_stack.is_empty() {
            self.inner
                .push(StackElement::ReturnAddress(self.next_instruction));
        }

        let mut bindings = Bindings::new();
        for (name, value) in arguments {
            bindings.insert(name, value);
        }
        self.inner.push(StackElement::Bindings(bindings));

        self.next_instruction = function.start;
        self.legacy_stack.push(());

        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<(), StackIsEmpty> {
        let mut index = self.inner.len();
        while index > 0 {
            index -= 1;

            if let StackElement::ReturnAddress(address) = self.inner[index] {
                self.next_instruction = address;
                self.inner.remove(index);
                break;
            }
            if let StackElement::Bindings(_) = self.inner[index] {
                self.inner.remove(index);
            }
        }

        let Some(_) = self.legacy_stack.pop() else {
            return Err(StackIsEmpty);
        };

        Ok(())
    }

    pub fn define_binding(&mut self, name: String, value: impl Into<Value>) {
        self.bindings_mut()
            .expect("Expected stack frame to exist")
            .insert(name, value.into());
    }

    pub fn push_operand(&mut self, operand: impl Into<Value>) {
        self.inner.push(StackElement::Operand(operand.into()));
    }

    pub fn pop_operand(&mut self) -> Result<Value, PopOperandError> {
        let mut index = self.inner.len();
        while index > 0 {
            index -= 1;

            if let StackElement::Operand(value) = self.inner[index] {
                self.inner.remove(index);
                return Ok(value);
            }
        }

        Err(PopOperandError::MissingOperand)
    }

    pub fn take_next_instruction(&mut self) -> Option<InstructionAddress> {
        if self.legacy_stack.is_empty() {
            return None;
        }

        let next_instruction = self.next_instruction;
        self.next_instruction.increment();

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
    Bindings(Bindings),
    Operand(Value),
    ReturnAddress(InstructionAddress),
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

    #[error("Evaluator is already finished")]
    Finished,
}

#[derive(Debug)]
pub struct StackIsEmpty;
