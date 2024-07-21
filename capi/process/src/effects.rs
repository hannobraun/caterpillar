use std::num::TryFromIntError;

use crate::{
    host::{GameEngineHost, Host},
    operands::PopOperandError,
    stack::PushStackFrameError,
};

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum Effect<H: Host> {
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

    #[error("Host-specific effect")]
    Host(H::Effect),
}

// This conversion is implemented manually, because doing it automatically using
// `thiserror`'s from would add an instance of the error into the type, and it
// doesn't implement `serde::Deserialize`.
impl From<TryFromIntError> for Effect<GameEngineHost> {
    fn from(_: TryFromIntError) -> Self {
        Self::OperandOutOfBounds
    }
}
