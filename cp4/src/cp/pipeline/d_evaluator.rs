use crate::cp::{DataStack, DataStackError};

use super::a_tokenizer::Tokenizer;

pub struct Evaluator {
    tokenizer: Tokenizer,
}

impl Evaluator {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    pub async fn evaluate(
        &mut self,
        data_stack: &mut DataStack,
    ) -> Result<(), EvaluatorError> {
        while let Some(token) = self.tokenizer.next_token().await {
            match token.as_str() {
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
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}
