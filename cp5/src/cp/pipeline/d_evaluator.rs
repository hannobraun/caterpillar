use crate::cp::{DataStack, DataStackError};

use super::a_tokenizer::Token;

pub fn evaluate(
    token: Token,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    let Token::Ident(word) = token;

    match word.as_str() {
        "true" => data_stack.push(true),
        "false" => data_stack.push(false),
        "not" => {
            let a = data_stack.pop()?;
            let x = !a;
            data_stack.push(x);
        }
        _ => return Err(EvaluatorError::UnknownWord(word)),
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
