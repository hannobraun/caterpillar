use super::Expression;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub expressions: Vec<Expression>,
}
