use async_recursion::async_recursion;

use crate::cp::{
    data_stack::Value,
    functions::Functions,
    syntax::{SyntaxElement, SyntaxSource},
    DataStack, DataStackError,
};

use super::b_parser::ParserError;

pub struct Evaluator {
    syntax: Box<dyn SyntaxSource>,
}

impl Evaluator {
    pub fn new(syntax: Box<dyn SyntaxSource>) -> Self {
        Self { syntax }
    }

    #[async_recursion(?Send)]
    pub async fn evaluate(
        &mut self,
        data_stack: &mut DataStack,
        functions: &mut Functions,
        tests: &mut Functions,
    ) -> Result<(), EvaluatorError> {
        loop {
            match self.syntax.next().await? {
                SyntaxElement::Block { syntax_tree } => {
                    let block = Value::Block(syntax_tree);
                    data_stack.push(block);
                }
                SyntaxElement::Function { name, body } => {
                    let module = String::new();
                    functions.define(name, module, body);
                }
                SyntaxElement::Word(word) => {
                    self.evaluate_word(word, data_stack, functions, tests)
                        .await?
                }
            }
        }
    }

    async fn evaluate_word(
        &mut self,
        word: String,
        data_stack: &mut DataStack,
        functions: &mut Functions,
        tests: &mut Functions,
    ) -> Result<(), EvaluatorError> {
        match word.as_str() {
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
            "eval" => {
                let a = data_stack.pop_block()?;
                Evaluator::new(Box::new(a))
                    .evaluate(data_stack, functions, tests)
                    .await?;
            }
            word => {
                if let Some(function) = functions.get(word) {
                    Evaluator::new(Box::new(function.body))
                        .evaluate(data_stack, functions, tests)
                        .await?;
                }

                return Err(EvaluatorError::UnknownWord(word.into()));
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
