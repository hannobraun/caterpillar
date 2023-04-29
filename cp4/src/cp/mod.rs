mod data_stack;

pub use self::data_stack::{DataStack, DataStackError};

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}

pub fn execute(code: &str) -> (Result<(), EvaluatorError>, DataStack) {
    let mut data_stack = DataStack::new();

    for code in code.split_whitespace() {
        match code {
            "true" => {
                data_stack.push(true);
            }
            "false" => {
                data_stack.push(false);
            }
            word => {
                return (
                    Err(EvaluatorError::UnknownWord(word.into())),
                    data_stack,
                )
            }
        }
    }

    (Ok(()), data_stack)
}
