pub struct DataStack {
    values: Vec<bool>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: bool) {
        self.values.push(value)
    }

    pub fn pop(&mut self) -> Result<bool, DataStackError> {
        self.values.pop().ok_or(DataStackError::PopFromEmptyStack)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataStackError {
    #[error("Tried to pop value from empty stack")]
    PopFromEmptyStack,
}
