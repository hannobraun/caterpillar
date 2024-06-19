use super::Value;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Operands {
    values: Vec<Value>,
}

impl Operands {
    pub fn new() -> Self {
        Self::default()
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

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("Tried to pop value from empty stack")]
pub struct StackUnderflow;
