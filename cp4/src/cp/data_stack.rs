use super::pipeline::b_parser::SyntaxTree;

#[derive(Debug)]
pub struct DataStack {
    values: Vec<Value>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    pub fn pop_bool(&mut self) -> Result<bool, DataStackError> {
        let value =
            self.values.pop().ok_or(DataStackError::PopFromEmptyStack)?;
        let Value::Bool(value) = value else {
            return Err(DataStackError::UnexpectedType { expected: "bool" });
        };
        Ok(value)
    }
}

#[derive(Debug)]
pub enum Value {
    Block(SyntaxTree),
    Bool(bool),
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

    #[error("Expected value of type {expected}")]
    UnexpectedType { expected: &'static str },
}
