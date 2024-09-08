use std::{collections::VecDeque, num::TryFromIntError};

use crate::{
    operands::PopOperandError, stack::PushStackFrameError,
    value::IntegerOverflow,
};

/// # The queue of unhandled effects
#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Effects {
    pub queue: VecDeque<Effect>,
}

impl Effects {
    /// # Trigger the provided effect
    ///
    /// This must not be called, while an effect is already triggered. Only call
    /// it from contexts, where it's known that no effect could be triggered, or
    /// right after handling the currently triggered effect.
    ///
    /// ## Panics
    ///
    /// Panics, if an effect is already triggered.
    pub fn trigger(&mut self, effect: impl Into<Effect>) {
        self.queue.push_back(effect.into());
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
    PopOperand(#[from] PopOperandError),

    #[error(transparent)]
    PushStackFrame(#[from] PushStackFrameError),

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
