use std::num::TryFromIntError;

use crate::{
    operands::PopOperandError, stack::PushStackFrameError,
    value::IntegerOverflow,
};

/// The queue of currently active effects
#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Effects {
    inner: Option<Effect>,
}

impl Effects {
    /// # Inspect the triggered effect
    pub fn inspect(&self) -> Option<&Effect> {
        self.inner.as_ref()
    }

    /// # Trigger the provided effect
    ///
    /// This must not be called, while an effect is already triggered. Only call
    /// it from contexts, where it's known that no effect could be triggered, or
    /// right after handling a currently triggered effect.
    ///
    /// ## Panics
    ///
    /// Panics, if an effect is already triggered.
    pub fn trigger(&mut self, effect: impl Into<Effect>) {
        assert!(
            self.inner.is_none(),
            "Trying to trigger an effect, while one is currently triggered. \
            This must never be done. That it still happened is a bug in \
            Caterpillar."
        );
        self.inner = Some(effect.into());
    }

    /// Handle the first effect in the queue
    ///
    /// If it can't be handled for some reason, which is probably a fatal
    /// failure, it should be re-triggered, to make sure all required
    /// information is available for debugging.
    pub fn handle_first(&mut self) -> Option<Effect> {
        self.inner.take()
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
