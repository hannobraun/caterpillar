use crate::cp::{DataStack, DataStackError};

use super::b_parser::{Parser, ParserError, SyntaxElement};

pub struct Evaluator {
    parser: Parser,
}

impl Evaluator {
    pub fn new(parser: Parser) -> Self {
        Self { parser }
    }

    pub async fn evaluate(
        &mut self,
        data_stack: &mut DataStack,
    ) -> Result<(), EvaluatorError> {
        while let Some(SyntaxElement::Word(token)) =
            self.parser.next_token().await?
        {
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
    Parser(#[from] ParserError),

    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}

impl EvaluatorError {
    pub fn is_no_more_chars(&self) -> bool {
        if let Self::Parser(err) = self {
            return err.is_no_more_chars();
        }

        false
    }
}
