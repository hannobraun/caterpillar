use capi_process::Value;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Expression {
    Binding {
        names: Vec<String>,
    },
    Block {
        expressions: Vec<Expression>,
    },
    Comment {
        text: String,
    },
    Reference {
        name: String,
        kind: Option<WordKind>,
    },
    Value(Value),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum WordKind {
    Binding,
    BuiltinFunction,
    HostFunction,
    UserFunction,
}
