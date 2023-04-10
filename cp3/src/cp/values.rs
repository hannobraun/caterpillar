use super::expressions::ExpressionGraph;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Value {
    Array(Vec<Value>),
    Block { expressions: ExpressionGraph },
    Bool(bool),
    String(String),
    U8(u8),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}
