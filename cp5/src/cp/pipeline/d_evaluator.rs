use crate::cp::{DataStack, DataStackError};

pub fn evaluate(
    word: &str,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    match word {
        "true" => data_stack.push(true),
        "false" => data_stack.push(false),
        "not" => {
            let a = data_stack.pop()?;
            let x = !a;
            data_stack.push(x);
        }
        _ => return Err(EvaluatorError::UnknownWord(word.into())),
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}
