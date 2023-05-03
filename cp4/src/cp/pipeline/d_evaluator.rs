use std::future::Future;

use crate::cp::{DataStack, DataStackError};

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}

pub async fn evaluate(
    token: impl Future<Output = String>,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    match token.await.as_str() {
        "true" => {
            data_stack.push(true);
        }
        "false" => {
            data_stack.push(false);
        }
        "not" => {
            let a = data_stack.pop_bool()?;
            let x = !a;
            data_stack.push(x);
        }
        word => return Err(EvaluatorError::UnknownWord(word.into())),
    }

    Ok(())
}
