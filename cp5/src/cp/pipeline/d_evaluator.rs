use crate::cp::{
    data_stack::Value, syntax::SyntaxElement, DataStack, DataStackError,
};

use super::{stage_input::StageInputReader, PipelineError};

pub fn evaluate(
    mut syntax_elements: StageInputReader<SyntaxElement>,
    data_stack: &mut DataStack,
) -> Result<(), PipelineError<EvaluatorError>> {
    evaluate_syntax_element(&mut syntax_elements, data_stack)?;
    syntax_elements.take();
    Ok(())
}

fn evaluate_syntax_element(
    syntax_elements: &mut StageInputReader<SyntaxElement>,
    data_stack: &mut DataStack,
) -> Result<(), PipelineError<EvaluatorError>> {
    match syntax_elements.next()? {
        SyntaxElement::Block { syntax_tree } => {
            data_stack.push(Value::Block(syntax_tree.clone()));
            Ok(())
        }
        SyntaxElement::Word(word) => {
            evaluate_word(word, data_stack).map_err(PipelineError::Stage)
        }
    }
}

fn evaluate_word(
    word: &str,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    match word {
        "true" => data_stack.push(true),
        "false" => data_stack.push(false),
        "not" => {
            let a = data_stack.pop_bool()?;
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

impl From<DataStackError> for PipelineError<EvaluatorError> {
    fn from(err: DataStackError) -> Self {
        PipelineError::Stage(err.into())
    }
}
