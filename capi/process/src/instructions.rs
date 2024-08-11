use std::{
    collections::{BTreeSet, VecDeque},
    fmt,
};

use crate::{Effect, Value};

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
    AssertBindingLeftNoOperands,
    Bind {
        name: String,
    },
    BindingEvaluate {
        name: String,
    },

    /// # Call a built-in function (or host function)
    ///
    /// Call a built-in function (or host function), by name.
    ///
    /// ## Implementation Note
    ///
    /// For historical reasons, this instructions lumps both built-in and host
    /// functions together. Since all that host functions do is trigger an
    /// effect (and the effect handler is responsible for anything else), it
    /// would make sense to just have a `TriggerEffect` instruction and directly
    /// compile host functions into those.
    ///
    /// Unfortunately, there are a few open questions about that. If the new
    /// instruction carried the actual effect type (either the full `Effect`, or
    /// an implementation of `HostEffect`), it would need to be generic over the
    /// host effect type. Since `Instruction` is a very basic type that is used
    /// in a lot of places, this would infect a lot of code with a type
    /// parameter that it otherwise doesn't care about.
    ///
    /// An alternative would be to encode the effect as a number. This actually
    /// seems like a decent solution, but it would have to be explored further.
    CallBuiltin {
        name: String,
    },

    /// # Call a cluster, selecting the right function via pattern matching
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
    CallCluster {
        cluster: Vec<(Vec<Pattern>, InstructionAddress)>,
        is_tail_call: bool,
    },

    MakeClosure {
        address: InstructionAddress,
        environment: BTreeSet<String>,
    },
    Push {
        value: Value,
    },
    Return,
    ReturnIfNonZero,
    ReturnIfZero,

    /// Trigger an effect
    TriggerEffect {
        effect: Effect,
    },
}

/// # A pattern in a function argument
///
/// ## Implementation Note
///
/// This duplicates the type of the same name in `capi-compiler`. This is
/// deliberate, as a shared type would have to live here (as `capi-compiler`
/// depends on this crate), but it doesn't belong here. The need for this type
/// is temporary, while so much of pattern matching is still figured out at
/// runtime.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Pattern {
    Identifier { name: String },
    Literal { value: Value },
}
