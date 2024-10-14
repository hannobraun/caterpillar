use capi_runtime::Value;

use crate::{
    fragments::UnresolvedCallToUserDefinedFunction,
    intrinsics::IntrinsicFunction,
};

use super::Function;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    CallToHostFunction {
        effect_number: u8,
    },

    CallToIntrinsicFunction {
        intrinsic: IntrinsicFunction,
        is_tail_call: bool,
    },

    Comment {
        text: String,
    },

    /// A function expression
    Function {
        function: Function,
    },

    ResolvedBinding {
        /// # The name of the binding
        name: String,
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

        /// # Indicate whether the identifier is known to be a function call
        ///
        /// This starts out as `false` and might later get updated by the
        /// respective compiler pass.
        is_known_to_be_call_to_user_defined_function:
            Option<UnresolvedCallToUserDefinedFunction>,
    },

    Value(Value),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentifierTarget {
    HostFunction { effect_number: u8 },
    Intrinsic { intrinsic: IntrinsicFunction },
}
