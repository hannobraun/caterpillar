use capi_process::Value;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Expression {
    Binding {
        names: Vec<String>,
    },
    Block {
        body: Vec<Expression>,
    },
    Comment {
        text: String,
    },

    /// A reference to a definition
    Reference {
        /// The name of the referenced definition, as it appears in the code
        name: String,

        /// The kind of referenced definition
        ///
        /// This might be `None`, while the kind of reference has not been
        /// determined yet, or if it can not be determined.
        kind: Option<ReferenceKind>,
    },

    Value(Value),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ReferenceKind {
    Binding,
    BuiltinFunction,
    HostFunction,
    UserFunction,
}
