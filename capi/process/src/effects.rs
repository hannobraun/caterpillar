use std::{collections::VecDeque, num::TryFromIntError};

use crate::{
    operands::PopOperandError, stack::PushStackFrameError,
    value::IntegerOverflow,
};

/// The queue of currently active effects
#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Effects {
    queue: VecDeque<Effect>,
}

impl Effects {
    /// Look at the first effect in the queue
    pub fn first(&self) -> Option<&Effect> {
        self.queue.front()
    }

    /// Handle the first effect in the queue
    ///
    /// If it can't be handled for some reason, which is probably a fatal
    /// failure, it should be re-triggered, to make sure all required
    /// information is available for debugging.
    pub fn handle_first(&mut self) -> Option<Effect> {
        self.queue.pop_front()
    }

    /// Trigger the provided effect
    ///
    /// The new effect is added to the front of the queue.
    pub fn trigger(&mut self, effect: impl Into<Effect>) {
        self.queue.push_front(effect.into());
    }
}

/// # An effect that interrupts code execution
///
/// Effects are produced when calling a host function, and can be produced by
/// various built-in functions, mostly (but not only) in case of an error.
///
/// Effects can be handled, which is common for host effects, which are designed
/// to pause the process and provide an opportunity for the host to interact
/// with it.
///
/// Other effects, error conditions, are meant to halt the process completely.
/// They can be displayed in the debugger, so the developer can learn what's
/// going on and fix their code accordingly.
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
    #[error("Binding expression left values on stack")]
    BindingLeftValuesOnStack,

    #[error("Breakpoint")]
    Breakpoint,

    #[error("Mis-compilation due to compiler bug")]
    CompilerBug,

    #[error("Divide by zero")]
    DivideByZero,

    #[error("Integer overflow")]
    IntegerOverflow,

    #[error("Invalid function")]
    InvalidFunction,

    #[error("Invalid host effect")]
    InvalidHostEffect,

    #[error("Missing `main` function")]
    MissingMainFunction,

    #[error("Pattern matching resulted in no match")]
    NoMatch,

    #[error("Operand is out of bounds")]
    OperandOutOfBounds,

    #[error(transparent)]
    PopOperand(#[from] PopOperandError),

    #[error(transparent)]
    PushStackFrame(#[from] PushStackFrameError),

    #[error("Unknown builtin")]
    UnknownBuiltin,

    #[error("Unknown host function")]
    UnknownHostFunction,

    #[error("Unresolved identifier")]
    UnresolvedIdentifier,

    /// A host-specific effect
    ///
    /// This host is expected to handle this effect. Any information it requires
    /// to do so, is expected to be present on the operand stack, when this
    /// effect is triggered.
    #[error("Host-specific effect")]
    Host,
}

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
