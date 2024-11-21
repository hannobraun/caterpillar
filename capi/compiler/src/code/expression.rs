use capi_runtime::Value;

use super::{Function, Hash};

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
    /// # A call to a user-defined function
    CallToUserDefinedFunction {
        /// # The hash of the function being called
        hash: Hash<Function>,
    },

    /// # A comment, which does not influence the execution of the code
    ///
    /// ## Implementation Note
    ///
    /// A comment is not really an expression, as it doesn't consume or produce
    /// any values. Thus, it's questionable whether it should be here.
    ///
    /// In addition, the comment being here, changes the hash of the function
    /// that it is in. This is unnecessary, as it would cause a function that
    /// hasn't actually changed to be considered different.
    ///
    /// For these reasons, it might be better to move comments into a separate
    /// data structure. However, so far, none of the above has been an actual
    /// problem. And for now, it's simpler to just treat comments as expressions
    /// that consume and produce nothing.
    ///
    /// Probably, it needs to move elsewhere, eventually. But there doesn't seem
    /// to be a compelling reasons to do that now.
    Comment {
        /// # The text of the comment
        text: String,
    },

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

    /// # A resolved local function
    ///
    /// Function literals start out as [`Expression::UnresolvedLocalFunction`].
    /// During compilation, they are replaced by this variant or
    /// [`Expression::LocalFunctionRecursive`] accordingly.
    ///
    /// ## Implementation Note
    ///
    /// As of this writing, this variant is not created yet.
    LocalFunction {
        /// # The hash of the local function
        ///
        /// ## Necessity
        ///
        /// Local functions can be accessed via their location in the code. So
        /// this hash is not needed for accessing this function.
        ///
        /// In fact, it _can't_ be needed for that purpose, as multiple compiler
        /// passes must run and have the need to access local functions, before
        /// functions are resolved and this hash exists.
        ///
        /// Despite this, this field is still required! Without it, there is
        /// nothing to distinguish expressions of this type, which means that
        /// distinct functions could end up with the same hash, despite being
        /// very different due to the local functions they define.
        hash: Hash<Function>,
    },

    /// # An unresolved local function
    ///
    /// This variant is created by the parser when it encounters a function
    /// literal. During compilation, it is replaced by either
    /// [`Expression::LocalFunction`] or [`Expression::LocalFunctionRecursive`],
    /// as appropriate.
    UnresolvedLocalFunction,
}

impl Expression {
    pub fn as_call_to_function(&self) -> Option<&Hash<Function>> {
        let Expression::CallToUserDefinedFunction { hash, .. } = self else {
            return None;
        };

        Some(hash)
    }

    pub fn as_comment(&self) -> Option<&String> {
        let Expression::Comment { text } = self else {
            return None;
        };

        Some(text)
    }
}
