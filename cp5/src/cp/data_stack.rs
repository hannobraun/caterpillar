use super::syntax::SyntaxTree;

#[derive(Debug)]
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

    pub fn pop_any(&mut self) -> Result<Value, DataStackError> {
        self.values.pop().ok_or(DataStackError::PopFromEmptyStack)
    }

    pub fn pop_bool(&mut self) -> Result<bool, DataStackError> {
        match self.pop_any()? {
            Value::Bool(value) => Ok(value),
            value => Err(DataStackError::UnexpectedType {
                expected: "bool",
                actual: value,
            }),
        }
    }

    pub fn pop_block(&mut self) -> Result<SyntaxTree, DataStackError> {
        match self.pop_any()? {
            Value::Block(value) => Ok(value),
            value => Err(DataStackError::UnexpectedType {
                expected: "block",
                actual: value,
            }),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Block(SyntaxTree),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataStackError {
    #[error("Tried to pop value from empty stack")]
    PopFromEmptyStack,

    #[error("Expected value of type `{expected}` but found {actual:?}")]
    UnexpectedType {
        expected: &'static str,
        actual: Value,
    },
}
