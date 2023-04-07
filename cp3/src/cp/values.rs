use super::expressions::ExpressionGraph;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Value {
    Array(Vec<Value>),
    Block { expressions: ExpressionGraph },
    Bool(bool),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
