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
