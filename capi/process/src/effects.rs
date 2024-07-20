use std::num::TryFromIntError;

use crate::{operands::PopOperandError, stack::PushStackFrameError};

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum EvaluatorEffect {
    #[error("Binding expression left values on stack")]
    BindingLeftValuesOnStack,

    #[error("Builtin effect: {self:?}")]
    Builtin(BuiltinEffect),

    #[error(transparent)]
    PopOperand(#[from] PopOperandError),

    #[error(transparent)]
    PushStackFrame(#[from] PushStackFrameError),

    #[error("Unknown builtin: {name}")]
    UnknownBuiltin { name: String },

    #[error("Executed unreachable instruction")]
    Unreachable,
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
pub enum BuiltinEffect {
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

    #[error("Host-specific effect")]
    Host(HostEffect),
}

// This conversion is implemented manually, because doing it automatically using
// `thiserror`'s from would add an instance of the error into the type, and it
// doesn't implement `serde::Deserialize`.
impl From<TryFromIntError> for BuiltinEffect {
    fn from(_: TryFromIntError) -> Self {
        Self::OperandOutOfBounds
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum HostEffect {
    Load { address: u8 },
    Store { address: u8, value: u8 },

    SetTile { x: u8, y: u8, color: [u8; 4] },
    SubmitFrame,

    ReadInput,
    ReadRandom,
}
