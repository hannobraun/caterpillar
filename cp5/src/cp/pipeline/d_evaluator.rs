use crate::cp::{DataStack, DataStackError};

use super::b_parser::SyntaxElement;

pub fn evaluate(
    syntax_element: SyntaxElement,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    let SyntaxElement::Word(word) = syntax_element;
    evaluator_word(word, data_stack)
}

fn evaluator_word(
    word: String,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
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
