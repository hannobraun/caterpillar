use super::{expressions::ExpressionGraph, values::Value};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DataStack {
    values: Vec<Value>,
    marker: usize,
}

impl DataStack {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            marker: 0,
        }
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into())
    }

    pub fn pop_any(&mut self) -> Result<Value, Error> {
        let value = self.values.pop();
        self.marker = usize::min(self.marker, self.values.len());
        value.ok_or(Error::PopFromEmptyStack)
    }

    pub fn pop_array(&mut self) -> Result<Vec<Value>, Error> {
        match self.pop_any()? {
            Value::Array(values) => Ok(values),
            value => Err(Error::UnexpectedType {
                expected: "array",
                actual: value,
            }),
        }
    }

    pub fn pop_bool(&mut self) -> Result<bool, Error> {
        match self.pop_any()? {
            Value::Bool(bool) => Ok(bool),
            value => Err(Error::UnexpectedType {
                expected: "bool",
                actual: value,
            }),
        }
    }

    pub fn pop_block(&mut self) -> Result<ExpressionGraph, Error> {
        match self.pop_any()? {
            Value::Block { expressions } => Ok(expressions),
            value => Err(Error::UnexpectedType {
                expected: "block",
                actual: value,
            }),
        }
    }

    pub fn pop_u8(&mut self) -> Result<u8, Error> {
        match self.pop_any()? {
            Value::U8(num) => Ok(num),
            value => Err(Error::UnexpectedType {
                expected: "u8",
                actual: value,
            }),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn mark(&mut self) {
        self.marker = self.values.len();
    }

    pub fn drain_values_from_mark(
        &mut self,
    ) -> impl Iterator<Item = Value> + '_ {
        self.values.drain(self.marker..)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
pub enum Error {
    #[error("Tried to pop value from empty stack")]
    PopFromEmptyStack,

    #[error("Expected value of type `{expected}` but found {actual:?}")]
    UnexpectedType {
        expected: &'static str,
        actual: Value,
    },
}
