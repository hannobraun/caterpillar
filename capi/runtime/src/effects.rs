use core::num::TryFromIntError;

use crate::{
    operands::PopOperandError, stack::PushStackFrameError,
    value::IntegerOverflow,
};

/// # A first-in, first-out queue of unhandled effects
#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct TriggeredEffect {
    queue: Option<Effect>,
}

impl TriggeredEffect {
    /// # Trigger the provided effect
    ///
    /// Triggering an effect adds it to the queue of unhandled effects. If
    /// there already are other unhandled effects in the queue, this new effect
    /// is added in last place.
    pub fn trigger(&mut self, effect: impl Into<Effect>) -> TriggerResult {
        if self.queue.is_none() {
            self.queue = Some(effect.into());
            TriggerResult::Triggered
        } else {
            TriggerResult::NotTriggeredBecauseTriggeredEffectAlreadyExists
        }
    }

    /// # Inspect the first effect in the queue
    ///
    /// Returns `None`, if the queue is empty.
    pub fn inspect_first(&self) -> Option<&Effect> {
        self.queue.as_ref()
    }

    /// # Handle the first effect in the queue
    ///
    /// Removes the first unhandled effect in the queue, considering it handled.
    ///
    /// Returns `None`, if the queue is empty.
    pub fn handle_first(&mut self) -> Option<Effect> {
        self.queue.take()
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
    snafu::Snafu,
)]
pub enum Effect {
    #[snafu(display("Breakpoint"))]
    Breakpoint,

    #[snafu(display(
        "Hit instruction that was generated from invalid Caterpillar code"
    ))]
    BuildError,

    #[snafu(display("Mis-compilation due to a compiler bug"))]
    CompilerBug,

    #[snafu(display("Divide by zero"))]
    DivideByZero,

    #[snafu(display("Integer overflow"))]
    IntegerOverflow,

    #[snafu(display("Invalid function"))]
    InvalidFunction,

    #[snafu(display("Invalid host effect"))]
    InvalidHostEffect,

    #[snafu(display("Pattern matching resulted in no match"))]
    NoMatch,

    #[snafu(display("Operand is out of bounds"))]
    OperandOutOfBounds,

    #[snafu(transparent)]
    PopOperand { source: PopOperandError },

    #[snafu(transparent)]
    PushStackFrame { source: PushStackFrameError },

    /// A host-specific effect
    ///
    /// This host is expected to handle this effect. Any information it requires
    /// to do so, is expected to be present on the operand stack, when this
    /// effect is triggered.
    #[snafu(display("Host-specific effect"))]
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

/// # The result of attempting to trigger an effect
///
/// Returned by [`Effects::trigger`].
#[must_use]
pub enum TriggerResult {
    /// # The effect has been triggered
    Triggered,

    /// # The effect has not been triggered
    ///
    /// A triggered effect already exists.
    NotTriggeredBecauseTriggeredEffectAlreadyExists,
}

impl TriggerResult {
    /// # Assert that the effect has been triggered
    ///
    /// This should be called by code that has previously inspected or handled
    /// the triggered effect and _knows_ that triggering an effect must succeed.
    ///
    /// ## Panics
    ///
    /// Panics, if the effect has not been triggered.
    pub fn assert_triggered(&self) {
        if !matches!(self, Self::Triggered) {
            panic!("Expected to trigger an effect, but that didn't happen.");
        }
    }

    /// # Ignore this result
    ///
    /// This should be called by code that does not actually care, if this
    /// specific effect has been triggered.
    pub fn ignore(self) {}
}
