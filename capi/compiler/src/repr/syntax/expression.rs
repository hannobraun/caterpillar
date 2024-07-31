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
    },

    Value(Value),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum IdentifierTarget {
    Binding,
    BuiltinFunction,
    HostFunction,
    UserFunction,
    SelfRecursion,
}
