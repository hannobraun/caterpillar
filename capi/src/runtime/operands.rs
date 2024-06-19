use super::Value;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Operands {
    values: Vec<Value>,
}

impl Operands {
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    pub fn pop(&mut self) -> Result<Value, MissingOperand> {
        self.values.pop().ok_or(MissingOperand)
    }

    pub fn values(&self) -> impl Iterator<Item = Value> + '_ {
        self.values.iter().copied()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("Missing operand")]
pub struct MissingOperand;
