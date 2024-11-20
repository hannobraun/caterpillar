use capi_runtime::Value;

use crate::intrinsics::IntrinsicFunction;

use super::{Function, FunctionLocation, Hash, Index};

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
    /// # A reference to a local binding
    Binding {
        /// # The name of the binding
        name: String,

        /// # The index of the binding
        ///
        /// The index is derived from the index of the binding in the parameter
        /// list of its branch. Only identifiers are counted.
        ///
        /// The index determines the position within the local stack frame,
        /// where the binding is stored.
        ///
        /// ## Implementation Note
        ///
        /// As of this writing, bindings are not actually stored like described
        /// here. This is a work in progress.
        index: u32,
    },

    /// # A call to a function defined by the host
    ///
    /// Host functions present as functions to the user. But contrary to regular
    /// functions, they have no representation in the form of Caterpillar code.
    ///
    /// The compiler translates calls to host functions into instructions that
    /// trigger a specific effect. This effect is then handled by the host in
    /// whatever way it deems appropriate.
    CallToHostFunction {
        /// # The number that identifies the host function
        number: u8,
    },

    /// # A call to a compiler-intrinsic function
    ///
    /// Intrinsic functions are implemented in the compiler. Calls to them are
    /// directly translated into a series of instructions, which provide the
    /// desired behavior.
    CallToIntrinsicFunction {
        /// # The intrinsic function being called
        intrinsic: IntrinsicFunction,

        /// # Indicate whether the call is in tail position
        ///
        /// This is relevant, as intrinsics can trigger calls to user-defined
        /// functions, which might necessitate tail call elimination.
        is_tail_call: bool,
    },

    /// # A call to a user-defined function
    CallToUserDefinedFunction {
        /// # The hash of the function being called
        hash: Hash<Function>,

        /// # Indicate whether the call is in tail position
        ///
        /// This is relevant as function calls might necessitate tail call
        /// elimination.
        is_tail_call: bool,
    },

    /// # A recursive call to a user-defined function
    ///
    /// This call can either be directly recursive (a function is calling
    /// itself), or mutually recursive (the function is calling another function
    /// that directly or indirectly calls the original function).
    ///
    /// This needs to be handled separately from non-recursive calls, as those
    /// non-recursive calls reference the callee by hash. In a recursive call,
    /// this is not possible. It would result in a circular dependency when
    /// creating the hash of the callee, since that would depend on the hash of
    /// caller, which would depend on the hash of the callee again.
    CallToUserDefinedFunctionRecursive {
        /// # The index of the called function within its cluster
        ///
        /// During compilation, functions are grouped into clusters. A cluster
        /// either contains a single functions, or a group of mutually recursive
        /// function. All mutually recursive functions are part of a single
        /// cluster.
        ///
        /// If this is a function calling itself, the index is always `0`. If
        /// the calling function is part of a cluster of mutually recursive
        /// functions, the index identifies the called function within the
        /// cluster.
        index: Index<FunctionLocation>,

        /// # Indicate whether the call is in tail position
        ///
        /// This is relevant as function calls might necessitate tail call
        /// elimination.
        is_tail_call: bool,
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

    /// # A resolved local function that calls a parent function recursively
    ///
    /// Function literals start out as [`Expression::UnresolvedLocalFunction`].
    /// During compilation, they are replaced by this variant or
    /// [`Expression::LocalFunction`] accordingly.
    ///
    /// The parent called might be the direct parent, or the parent of any
    /// parent, recursively.
    ///
    /// The call to the parent itself might be direct, or it might be indirect,
    /// occurring in a local function defined in this function, or any of their
    /// local functions, recursively.
    ///
    /// This variant is distinct from [`Expression::LocalFunction`], as local
    /// functions that call their parent can't be hashed. The hash of the local
    /// function and the parent it calls would depend on each other, which can't
    /// work.
    ///
    /// ## Implementation Note
    ///
    /// As of this writing, this variant is not created yet.
    LocalFunctionRecursive {
        /// # The index of the local function within its cluster
        ///
        /// ## Necessity
        ///
        /// Local functions can be accessed via their location in the code. So
        /// this index is not needed for accessing this function.
        ///
        /// In fact, it _can't_ be needed for that purpose, as multiple compiler
        /// passes must run and have the need to access local functions, before
        /// functions are resolved and this index exists.
        ///
        /// Despite this, this field is still required! Without it, there is
        /// nothing to distinguish expressions of this type, which means that
        /// distinct functions could end up with the same hash, despite being
        /// very different due to the local functions they define.
        index: Index<FunctionLocation>,
    },

    /// # An unresolved identifier
    ///
    /// This is the result of a compiler error.
    UnresolvedIdentifier {
        /// # The name of the unresolved identifier
        name: String,

        /// # Indicate whether the identifier is known to be a function call
        ///
        /// This starts out as `false` and might later get updated by the
        /// respective compiler pass.
        is_known_to_be_call_to_user_defined_function: bool,
    },

    /// # An unresolved local function
    ///
    /// This variant is created by the parser when it encounters a function
    /// literal. During compilation, it is replaced by either
    /// [`Expression::LocalFunction`] or [`Expression::LocalFunctionRecursive`],
    /// as appropriate.
    ///
    /// ## Implementation Note
    ///
    /// The replacement described above does not happen yet, as of this writing.
    /// There is an ongoing cleanup effort with the goal of changing that.
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
