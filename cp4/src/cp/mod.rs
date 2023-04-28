#[derive(Debug)]
pub struct DataStack {
    values: Vec<bool>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn pop_bool(&mut self) -> Result<bool, DataStackError> {
        self.values.pop().ok_or(DataStackError::PopFromEmptyStack)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataStackError {
    #[error("Tried to pop value from empty stack")]
    PopFromEmptyStack,
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {}

pub fn execute(_: &str) -> (Result<(), EvaluatorError>, DataStack) {
    let result = Ok(());
    let data_stack = DataStack::new();

    (result, data_stack)
}
