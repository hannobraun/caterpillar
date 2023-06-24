use super::data_stack::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expressions {
    pub elements: Vec<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Array { expressions: Expressions },
    Binding { idents: Vec<String> },
    EvalBinding { name: String },
    EvalFunction { name: String },
    Module { name: String, body: Expressions },
    Value(Value),
}
