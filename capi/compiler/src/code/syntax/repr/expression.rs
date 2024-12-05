use capi_runtime::Value;

use super::functions::Function;

/// # An expression within a function
///
/// ## Error Handling
///
/// An important feature of Caterpillar's code representation is, that it can be
/// the result of a failed compilation process. If, for example, an identifier
/// can't be resolved, this is still encoded as a valid [`Expression`].
///
/// As a result, other code that is not affected can still be executed (as part
/// of automated testing, for example). But also, the rich representation
/// produced by the compilation process is still available for display by
/// tooling, regardless of any isolated errors.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub enum Expression {
    /// # An identifier
    ///
    /// Can refer to a binding or function.
    Identifier {
        /// # The name of the identifier
        name: String,
    },

    /// # A number literal
    LiteralNumber {
        /// The number defined by this literal
        value: Value,
    },

    /// # A local function
    LocalFunction {
        /// # The local function
        function: Function,
    },
}

impl Expression {
    /// # Convert the expression into a local function
    ///
    /// Returns `None`, if this expression is not a local function.
    pub fn as_local_function(&self) -> Option<&Function> {
        let Expression::LocalFunction { function } = self else {
            return None;
        };

        Some(function)
    }
}
