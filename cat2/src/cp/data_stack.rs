use super::Expressions;

#[derive(Debug)]
pub struct DataStack {
    inner: Vec<Value>,
}

impl DataStack {
    pub fn new() -> Self {
        let inner = Vec::new();
        Self { inner }
    }

    pub fn pop_any(&mut self) -> Value {
        self.inner.pop().expect("Stack is empty")
    }

    pub fn pop_bool(&mut self) -> bool {
        let Value::Bool(value) = self.pop_any() else {
            panic!("Expected `bool`")
        };
        value
    }

    pub fn pop_u8(&mut self) -> u8 {
        let Value::U8(value) = self.pop_any() else {
            panic!("Expected `u8`")
        };
        value
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.inner.push(value.into())
    }

    pub fn values_from_top(&self) -> impl Iterator<Item = &Value> {
        self.inner.iter().rev()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    Block(Block),
    Bool(bool),
    U8(u8),
}

impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Self::Block(_) => Type::Block,
            Self::Bool(_) => Type::Bool,
            Self::U8(_) => Type::U8,
        }
    }
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

pub type Block = Expressions;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    Block,
    Bool,
    U8,
}
