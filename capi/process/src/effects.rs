use std::num::TryFromIntError;

use crate::{
    operands::PopOperandError, stack::PushStackFrameError,
    value::IntegerOverflow,
};

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
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    thiserror::Error,
)]
pub enum Effect<H> {
    #[error(transparent)]
    Core(CoreEffect),

    #[error("Host-specific effect")]
    Host(H),

    /// A host-specific effect
    ///
    /// This host is expected to handle this effect. Any information it requires
    /// to do so, is expected to be present on the operand stack, when this
    /// effect is triggered.
    #[error("Host-specific effect")]
    Host2,
}

impl<T, H> From<T> for Effect<H>
where
    T: Into<CoreEffect>,
{
    fn from(value: T) -> Self {
        Self::Core(value.into())
    }
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    thiserror::Error,
)]
pub enum CoreEffect {
    #[error("Binding expression left values on stack")]
    BindingLeftValuesOnStack,

    #[error("Breakpoint")]
    Breakpoint,

    #[error("Divide by zero")]
    DivideByZero,

    #[error(transparent)]
    IntegerOverflow(#[from] IntegerOverflow),

    #[error("Pattern matching resulted in no match")]
    NoMatch,

    #[error("Operand is out of bounds")]
    OperandOutOfBounds,

    #[error(transparent)]
    PopOperand(#[from] PopOperandError),

    #[error(transparent)]
    PushStackFrame(#[from] PushStackFrameError),

    #[error("Unknown builtin: {name}")]
    UnknownBuiltin { name: String },

    #[error("Panic")]
    Panic,
}

// This conversion is implemented manually, because doing it automatically using
// `thiserror`'s `#[from]` would add an instance of the error into the type, and
// it doesn't implement `serde::Deserialize`.
impl From<TryFromIntError> for CoreEffect {
    fn from(_: TryFromIntError) -> Self {
        Self::OperandOutOfBounds
    }
}
