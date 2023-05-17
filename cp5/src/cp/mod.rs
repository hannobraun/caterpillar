mod data_stack;

pub use self::data_stack::DataStackError;

pub fn execute(code: &str) -> bool {
    code == "true"
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {}
