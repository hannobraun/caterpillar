use capi_process::Value;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Expression {
    Binding { names: Vec<String> },
    Comment { text: String },
    Value(Value),
    Word { name: String },
}
