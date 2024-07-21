use std::num::TryFromIntError;

use crate::{
    host::Host, operands::PopOperandError, stack::PushStackFrameError,
};

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    thiserror::Error,
)]
pub enum Effect<H: Host> {
    #[error(transparent)]
    Core(CoreEffect),

    #[error("Host-specific effect")]
    Host(H::Effect),
}

impl<T, H: Host> From<T> for Effect<H>
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

    #[error("Integer overflow")]
    IntegerOverflow,

    #[error("Operand is out of bounds")]
    OperandOutOfBounds,

    #[error(transparent)]
    PopOperand(#[from] PopOperandError),

    #[error(transparent)]
    PushStackFrame(#[from] PushStackFrameError),

    #[error("Unknown builtin: {name}")]
    UnknownBuiltin { name: String },

    #[error("Executed unreachable instruction")]
    Unreachable,
}

// This conversion is implemented manually, because doing it automatically using
// `thiserror`'s from would add an instance of the error into the type, and it
// doesn't implement `serde::Deserialize`.
impl From<TryFromIntError> for CoreEffect {
    fn from(_: TryFromIntError) -> Self {
        Self::OperandOutOfBounds
    }
}
