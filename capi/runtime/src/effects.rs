use std::{collections::VecDeque, num::TryFromIntError};

use crate::{
    operands::PopOperandError, stack::PushStackFrameError,
    value::IntegerOverflow,
};

/// # A first-in, first-out queue of unhandled effects
#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Effects {
    queue: VecDeque<Effect>,
}

impl Effects {
    /// # Trigger the provided effect
    ///
    /// Triggering an effect adds it to the queue of unhandled effects. If
    /// there already are other unhandled effects in the queue, this new effect
    /// is added in last place.
    pub fn trigger(&mut self, effect: impl Into<Effect>) {
        self.queue.push_back(effect.into());
    }

    /// # Inspect the first effect in the queue
    ///
    /// Returns `None`, if the queue is empty.
    pub fn inspect_first(&self) -> Option<&Effect> {
        self.queue.front()
    }

    /// # Handle the first effect in the queue
    ///
    /// Removes the first unhandled effect in the queue, considering it handled.
    ///
    /// Returns `None`, if the queue is empty.
    pub fn handle_first(&mut self) -> Option<Effect> {
        self.queue.pop_front()
    }

    /// # Iterate over all effects in the queue
    pub fn queue(&self) -> impl Iterator<Item = Effect> + '_ {
        self.queue.iter().copied()
    }
}

/// # An effect that interrupts evaluation
///
/// Effects can be triggered when instructions are executed. Most of them
/// represent error conditions, but some are used for debugging
/// ([`Effect::Breakpoint`]) or communication with the host ([`Effect::Host`]).
///
/// Effects can be handled, after which evaluation can resume. This is common
/// for host effects, which are designed to provide an opportunity for the host
/// to interact with the process.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    thiserror::Error,
)]
pub enum Effect {
    #[error("Breakpoint")]
    Breakpoint,

    #[error(
        "Hit instruction that was generated from invalid Caterpillar code"
    )]
    BuildError,

    #[error("Mis-compilation due to a compiler bug")]
    CompilerBug,

    #[error("Divide by zero")]
    DivideByZero,

    #[error("Integer overflow")]
    IntegerOverflow,

    #[error("Invalid function")]
    InvalidFunction,

    #[error("Invalid host effect")]
    InvalidHostEffect,

    #[error("Pattern matching resulted in no match")]
    NoMatch,

    #[error("Operand is out of bounds")]
    OperandOutOfBounds,

    #[error(transparent)]
    PopOperand {
        #[from]
        source: PopOperandError,
    },

    #[error(transparent)]
    PushStackFrame {
        #[from]
        source: PushStackFrameError,
    },

    /// A host-specific effect
    ///
    /// This host is expected to handle this effect. Any information it requires
    /// to do so, is expected to be present on the operand stack, when this
    /// effect is triggered.
    #[error("Host-specific effect")]
    Host,
}

// This conversion is implemented manually, because doing it automatically using
// `thiserror`'s `#[from]` would add an instance of the error into the type.
// This makes the respective effect more complex to construct manually.
impl From<IntegerOverflow> for Effect {
    fn from(_: IntegerOverflow) -> Self {
        Self::IntegerOverflow
    }
}

// This conversion is implemented manually, because doing it automatically using
// `thiserror`'s `#[from]` would add an instance of the error into the type, and
// it doesn't implement `serde::Deserialize`.
impl From<TryFromIntError> for Effect {
    fn from(_: TryFromIntError) -> Self {
        Self::OperandOutOfBounds
    }
}
