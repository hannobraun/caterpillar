#[derive(Debug)]
pub struct DataStack {}

impl DataStack {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn pop_bool(&mut self) -> Result<bool, DataStackError> {
        Ok(false)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataStackError {}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {}

pub fn execute(_: &str) -> (Result<(), EvaluatorError>, DataStack) {
    let result = Ok(());
    let data_stack = DataStack::new();

    (result, data_stack)
}
