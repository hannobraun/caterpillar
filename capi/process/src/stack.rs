use std::collections::BTreeMap;

use crate::{operands::PopOperandError, InstructionAddress, Value};

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

    pub next_instruction: InstructionAddress,

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
}

impl Stack {
    pub fn new() -> Self {
        Self {
            inner: vec![
                StackElement::StartMarker,
                StackElement::Bindings(Bindings::new()),
            ],
            next_instruction: InstructionAddress { index: 0 },
            closures: BTreeMap::new(),
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
                StackElement::StartMarker => None,
            })
            .chain([self.next_instruction])
    }

    pub fn push_frame(
        &mut self,
        arguments: Vec<(String, Value)>,
    ) -> Result<(), PushStackFrameError> {
        // Not a tail call. This means we need to create a new stack frame.
        // Let's first check if we can even do that.
        const STACK_LIMIT: usize = 16;
        if self.inner.len() >= STACK_LIMIT {
            return Err(PushStackFrameError::Overflow);
        }

        // All stack frames but the initial one (which this one can't be, as the
        // initial one is created with the stack), start with a return address.
        self.inner
            .push(StackElement::ReturnAddress(self.next_instruction));

        // And all stack frames need a map of bindings.
        let bindings = arguments.into_iter().collect();
        self.inner.push(StackElement::Bindings(bindings));

        Ok(())
    }

    pub fn reuse_frame(&mut self) {
        // We are repurposing the existing stack frame.
        //
        // This means the element that marks the start of the stack frame,
        // either the start marker or a return address, can stay as they are.
        //
        // So can operands. Those that are function arguments, we already
        // removed. Those that remain are what the tail-calling function
        // returns, so they can remain and be returned when the stack frame is
        // eventually done.
        //
        // But we need to handle bindings.

        let bindings = self.bindings_mut().expect(
            "Until the process has finished, there is always a stack frame. \
            Either the initial one, or one that was pushed while the process \
            was running.\n\
            \n\
            A new stack frame is being pushed right now, hence there must be \
            an existing one, which means it must be possible to find bindings.",
        );

        // Any bindings that remain are no longer accessible, so let's remove
        // them.
        bindings.clear();
    }

    pub fn pop_frame(&mut self) {
        let mut index = self.inner.len();

        loop {
            if index == 0 {
                break;
            }

            index -= 1;

            match self.inner[index] {
                StackElement::Bindings(_) => {
                    self.inner.remove(index);
                }
                StackElement::ReturnAddress(address) => {
                    self.next_instruction = address;
                    self.inner.remove(index);
                    break;
                }
                StackElement::StartMarker => {
                    self.inner.remove(index);
                    break;
                }
                _ => {}
            }
        }
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
        if self.inner.is_empty() {
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

/// # The things that can be on the stack
///
/// ## Implementation Note
///
/// This is an enum, because right now a lot of things still happen at runtime,
/// and the stack logic needs to recognize what kind of element it's looking at
/// to make an informed decision.
///
/// Eventually, the compiler will grow smarter, and be able to figure out what
/// needs to happen with the stack at compile-time. At that point, we will no
/// longer need to track this kind of type information at runtime.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
enum StackElement {
    /// # The bindings in the current stack frame
    ///
    /// There should be one of these per stack frame, and it's expected that
    /// this is the first element after the return address.
    ///
    /// ## Implementation Note
    ///
    /// Having a map with the names and values of bindings at runtime is
    /// unnecessary. We need the value, sure, but the compiler can track the
    /// names and locations of bindings at compile-time.
    ///
    /// At some point it will learn to do that, and generate the right
    /// instructions to access them. Then we won't need to look up bindings by
    /// name at runtime.
    Bindings(Bindings),

    /// An operand
    Operand(Value),

    /// A return address
    ///
    /// This marks the beginning of a stack frame. It carries the address that
    /// the evaluator needs to jump back to, once it's done with the current
    /// stack frame.
    ReturnAddress(InstructionAddress),

    /// A marker to substitute the return address in the initial stack frame
    ///
    /// The initial stack frame needs no return address, so it has this marker.
    /// The reason we need it, is to know when the first stack frame is being
    /// dropped, which indicates that the process has finished.
    ///
    /// Without a start marker, when we pop a frame, we wouldn't be able to
    /// distinguish whether the process has finished, or if we just happen to
    /// have an empty stack because of tail call elimination, but should still
    /// continue running.
    StartMarker,
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
