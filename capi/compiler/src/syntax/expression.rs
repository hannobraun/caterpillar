use capi_runtime::Value;

use crate::{fragments::FunctionIndexInCluster, intrinsics::Intrinsic};

use super::Function;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Comment {
        text: String,
    },

    /// A function expression
    Function {
        function: Function,
    },

    /// A name that identifies a definition in the source code
    Identifier {
        /// The name of the definition, as it appears in the code
        name: String,

        /// The kind of definition that the identifier identifies
        ///
        /// This might be `None`, if the target has not been determined yet, or
        /// can not be determined.
        target: Option<IdentifierTarget>,

        /// Indicate whether the identifier is known to be in tail position
        ///
        /// An expression is in tail position, if it is the last expression in
        /// its function or block.
        ///
        /// This starts out being `false` for all expressions, and will
        /// eventually be filled in by a dedicated compiler pass.
        ///
        /// This flag is relevant for tail call elimination. It is only needed
        /// for identifiers, because only identifiers can lead to tail calls.
        is_known_to_be_in_tail_position: bool,
    },

    Value(Value),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentifierTarget {
    Binding,

    Function {
        /// # Index of function within cluster, if this is a recursive call
        ///
        /// This covers both self-recursive calls (a function calls itself), as
        /// well as mutually recursive calls (a function calls another function
        /// that directly or indirectly calls the original function).
        ///
        /// This starts as `None`, until the respective compiler pass has has
        /// run.
        is_known_to_be_recursive_call_to_index: Option<FunctionIndexInCluster>,
    },

    HostFunction {
        effect_number: u8,
    },

    Intrinsic {
        intrinsic: Intrinsic,
    },
}
