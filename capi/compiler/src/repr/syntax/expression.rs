use capi_process::Value;

use crate::repr::syntax::Location;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Expression {
    pub kind: ExpressionKind,
}

impl Expression {
    pub fn new(kind: ExpressionKind, _: Location) -> Self {
        Self { kind }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ExpressionKind {
    Binding { names: Vec<String> },
    Comment { text: String },
    Value(Value),
    Word { name: String },
}
