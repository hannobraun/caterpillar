use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct DataStack {
    values: Vec<Value>,
}

impl DataStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    pub fn pop(&mut self) -> Result<Value, StackUnderflow> {
        self.values.pop().ok_or(StackUnderflow)
    }

    pub fn values(&self) -> impl Iterator<Item = Value> + '_ {
        self.values.iter().copied()
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Value(pub i8);

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("Tried to pop value from empty stack")]
pub struct StackUnderflow;
