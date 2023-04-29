mod data_stack;

pub use self::data_stack::{DataStack, DataStackError};

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}

pub fn execute(
    code: &str,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    for code in code.split_whitespace() {
        match code {
            "true" => {
                data_stack.push(true);
            }
            "false" => {
                data_stack.push(false);
            }
            word => return Err(EvaluatorError::UnknownWord(word.into())),
        }
    }

    Ok(())
}
