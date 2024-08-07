use std::collections::BTreeSet;

use capi_process::Value;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Expression {
    Binding {
        names: Vec<String>,
    },

    /// A block of code
    Block {
        /// The body of the block
        body: Vec<Expression>,

        /// The block's environment
        ///
        /// These are the bindings defined in parent blocks that are referenced
        /// from the block's body.
        ///
        /// The environment is empty on construction, until it is filled in
        /// during the resolve pass.
        environment: BTreeSet<String>,
    },

    Comment {
        text: String,
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

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum IdentifierTarget {
    Binding,
    BuiltinFunction,
    Cluster,
    HostFunction,
}
