use super::Expression;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub body: Vec<Expression>,
}

pub enum Pattern {
    Identifier { name: String },
}
