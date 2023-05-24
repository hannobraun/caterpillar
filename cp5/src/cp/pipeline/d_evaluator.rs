use crate::cp::{
    data_stack::Value, syntax::SyntaxElement, DataStack, DataStackError,
    Functions,
};

use super::{stage_input::StageInputReader, PipelineError};

pub fn evaluate(
    mut syntax_elements: StageInputReader<SyntaxElement>,
    data_stack: &mut DataStack,
    functions: &mut Functions,
) -> Result<(), PipelineError<EvaluatorError>> {
    let syntax_element = syntax_elements.next()?;
    evaluate_syntax_element(syntax_element, data_stack, functions)?;
    syntax_elements.take();
    Ok(())
}

fn evaluate_syntax_element(
    syntax_element: &SyntaxElement,
    data_stack: &mut DataStack,
    functions: &mut Functions,
) -> Result<(), PipelineError<EvaluatorError>> {
    match syntax_element {
        SyntaxElement::Block { syntax_tree } => {
            data_stack.push(Value::Block(syntax_tree.clone()));
            Ok(())
        }
        SyntaxElement::Function { name, body } => {
            functions.define(name.clone(), body.clone());
            Ok(())
        }
        SyntaxElement::Word(word) => evaluate_word(word, data_stack, functions),
    }
}

fn evaluate_word(
    word: &str,
    data_stack: &mut DataStack,
    functions: &mut Functions,
) -> Result<(), PipelineError<EvaluatorError>> {
    match word {
        "true" => data_stack.push(true),
        "false" => data_stack.push(false),
        "not" => {
            let b = data_stack.pop_bool()?;
            data_stack.push(!b);
        }
        "eval" => {
            let block = data_stack.pop_block()?;
            for syntax_element in block.elements {
                evaluate_syntax_element(
                    &syntax_element,
                    data_stack,
                    functions,
                )?;
            }
        }
        _ => {
            return Err(PipelineError::Stage(EvaluatorError::UnknownWord(
                word.into(),
            )));
        }
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

impl From<DataStackError> for PipelineError<EvaluatorError> {
    fn from(err: DataStackError) -> Self {
        PipelineError::Stage(err.into())
    }
}
