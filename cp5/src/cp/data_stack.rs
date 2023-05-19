pub struct DataStack {
    values: Vec<Value>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: bool) {
        self.values.push(value.into())
    }

    pub fn pop_bool(&mut self) -> Result<bool, DataStackError> {
        let value =
            self.values.pop().ok_or(DataStackError::PopFromEmptyStack)?;
        match value {
            Value::Bool(value) => Ok(value),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug)]
pub enum Value {
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
}
