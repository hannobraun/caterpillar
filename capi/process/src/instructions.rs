use std::{
    collections::{BTreeSet, VecDeque},
    fmt,
};

use crate::{Branch, Effect, Function, Value};

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Instructions {
    inner: InstructionsInner,
}

impl Instructions {
    pub fn push(&mut self, instruction: Instruction) -> InstructionAddress {
        let address = InstructionAddress {
            index: self.inner.len().try_into().unwrap(),
        };
        self.inner.push_back((address, instruction));
        address
    }

    pub fn get(&self, address: &InstructionAddress) -> Option<&Instruction> {
        let (stored_address, instruction) =
            self.inner.get(address.to_usize())?;
        assert_eq!(address, stored_address);
        Some(instruction)
    }

    pub fn replace(
        &mut self,
        address: InstructionAddress,
        instruction: Instruction,
    ) {
        let (stored_address, stored_instruction) =
            self.inner.get_mut(address.to_usize()).unwrap();
        assert_eq!(address, *stored_address);
        *stored_instruction = instruction;
    }
}

impl<'r> IntoIterator for &'r Instructions {
    type Item = <&'r InstructionsInner as IntoIterator>::Item;
    type IntoIter = <&'r InstructionsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

type InstructionsInner = VecDeque<(InstructionAddress, Instruction)>;

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct InstructionAddress {
    pub index: u32,
}

impl InstructionAddress {
    pub fn increment(&mut self) {
        self.index += 1;
    }

    fn to_usize(self) -> usize {
        self.index
            .try_into()
            .expect("Expected `usize` to cover full range of `u32`")
    }
}

impl fmt::Display for InstructionAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.index)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Instruction {
    /// # Add two signed 8-bit integers, triggering an error on overflow
    AddS8,

    /// # Add two signed 32-bit integers, triggering an error on overflow
    AddS32,

    /// # Add two unsigned 8-bit integers, triggering an error on overflow
    AddU8,

    /// # Add two unsigned 8-bit integers, wrapping on overflow
    AddU8Wrap,

    Bind {
        name: String,
    },
    BindingEvaluate {
        name: String,
    },

    /// # Call a built-in function by name
    ///
    /// ## Implementation Note
    ///
    /// With the addition of compiler intrinsics, this instruction is on its way
    /// out. Compiler intrinsics can, in principle, offer the same things as
    /// built-in functions do, but do so in a more appropriate way. They move
    /// the required smarts into the compiler, allowing the evaluator to be
    /// simpler.
    CallBuiltin {
        name: String,
    },

    /// # Call a function, selecting the right branch via pattern matching
    ///
    /// ## Implementation Note
    ///
    /// This instruction is overly complex. I think ideally, there would be two
    /// simple instructions instead:
    ///
    /// - `PushFrame`, to push a new stack frame, and do nothing else with it.
    /// - `ReuseFrame`, to prepare the existing stack frame for reuse. (If
    ///   bindings were just regular values whose addresses were managed by the
    ///   compiler, instead of being a special thing handled by the stack at
    ///   runtime, then we probably wouldn't need this at all, and could reuse
    ///   any stack frame without preparation. That's a different story,
    ///   however.)
    ///
    /// Those would just take a single address as an argument, and jump there
    /// directly. (And maybe even pushing a frame and jumping to an instructions
    /// could and should be decoupled eventually.)
    ///
    /// Pattern matching could move into the functions themselves. The compiler
    /// could generate the necessary code as required, using the following
    /// approach:
    ///
    /// - If there are no literal patterns among the arguments, just generate
    ///   nothing.
    /// - If there are literal patterns, generate code that compares them
    ///   against the call stack.
    ///   - If all patterns match, remove the respective operands from the
    ///     stack and continue with the rest of the function as normal.
    ///     (Although probably the next step would be the compiler-generated
    ///     code that handles the arguments.
    ///   - If the patterns don't match, leave the operands as-is and jump
    ///     directly to the next function, which continues with its own pattern
    ///     matching.
    ///
    /// This would require some new instructions for doing the comparison and
    /// jumping to another address, but that seems like the right direction to
    /// go in anyway.
    CallFunction {
        function: Function,
        is_tail_call: bool,
    },

    /// # Convert a signed 32-bit number to a signed 8-bit number
    ConvertS32ToS8,

    /// # Copy a value,
    Copy,

    /// # Divide two signed 32-bit integers
    DivS32,

    /// # Divide two unsigned 8-bit integers
    DivU8,

    /// # Drop a value
    Drop,

    /// # Compare two values for equality
    Eq,

    /// # Evaluate the current function on the stack
    ///
    /// ## Implementation Note
    ///
    /// This instruction is too high-level, and it's partially redundant with
    /// other high-level instructions. The duplicated code in their
    /// implementations within `Evaluator` is supporting evidence of this.
    ///
    /// Like other instructions, it needs to be replaced by smaller, more
    /// low-level ones. This requires the compiler to become smarter. Which is
    /// the direction I'd like things to go into anyway, but it hasn't fully
    /// happened yet.
    Eval {
        is_tail_call: bool,
    },

    /// # Determine if the first of two signed 8-bit numbers is greater
    GreaterS8,

    /// # Determine if the first of two signed 32-bit numbers is greater
    GreaterS32,

    /// # Determine if the first of two unsigned 8-bit numbers is greater
    GreaterU8,

    /// # Logical and
    LogicalAnd,

    /// # Logical not
    LogicalNot,

    MakeClosure {
        branches: Vec<Branch>,
        environment: BTreeSet<String>,
    },

    /// # Multiply two signed 32-bit numbers, triggering an error on overflow
    MulS32,

    /// # Multiply two unsigned 8-bit numbers, wrapping on overflow
    MulU8Wrap,

    /// # Negate a signed 32-bit number
    NegS32,

    Push {
        value: Value,
    },

    /// # Compute the remainder of the division of two signed 32-bit numbers
    RemainderS32,

    Return,

    /// # Subtract two signed 32-bit numbers, triggering an error on overflow
    SubS32,

    /// # Subtract two unsigned 8-bit numbers, triggering an error on overflow
    SubU8,

    /// # Subtract two unsigned 8-bit numbers, wrapping on overflow
    SubU8Wrap,

    /// Trigger an effect
    TriggerEffect {
        effect: Effect,
    },
}
