use super::parser::Expressions;

pub struct DataStack {
    values: Vec<Value>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into())
    }

    pub fn pop_any(&mut self) -> Result<Value, Error> {
        self.values.pop().ok_or(Error::PopFromEmptyStack)
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

    pub fn pop_block(&mut self) -> Result<Expressions, Error> {
        match self.pop_any()? {
            Value::Block(expressions) => Ok(expressions),
            value => Err(Error::UnexpectedType {
                expected: "block",
                actual: value,
            }),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tried to pop value from empty stack")]
    PopFromEmptyStack,

    #[error("Expected value of type `{expected}` but found {actual:?}")]
    UnexpectedType {
        expected: &'static str,
        actual: Value,
    },
}

#[derive(Debug)]
pub enum Value {
    Block(Expressions),
    Bool(bool),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
