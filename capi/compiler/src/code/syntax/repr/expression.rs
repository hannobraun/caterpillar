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

    /// # A local function
    LocalFunction {
        /// # The local function
        function: Function,
    },
}

impl Expression {
    /// # Convert the expression into a comment
    ///
    /// Returns `None`, if this expression is not a comment.
    pub fn as_comment(&self) -> Option<&String> {
        let Expression::Comment { text } = self else {
            return None;
        };

        Some(text)
    }

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
