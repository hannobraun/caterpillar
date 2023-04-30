mod data_stack;
mod pipeline;

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
    for token in pipeline::a_tokenizer::tokenize(code) {
        match token {
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
    }

    Ok(())
}
