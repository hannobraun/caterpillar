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

    pub fn push(&mut self, value: bool) {
        self.values.push(value);
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
pub enum EvaluatorError {
    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}

pub fn execute(code: &str) -> (Result<(), EvaluatorError>, DataStack) {
    let mut data_stack = DataStack::new();

    let (value, result) = match code {
        "true" => (true, Ok(())),
        "false" => (false, Ok(())),
        word => {
            return (Err(EvaluatorError::UnknownWord(word.into())), data_stack)
        }
    };

    data_stack.push(value);

    (result, data_stack)
}
