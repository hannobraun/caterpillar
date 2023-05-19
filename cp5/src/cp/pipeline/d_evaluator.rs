use crate::cp::{
    data_stack::Value, syntax::SyntaxElement, DataStack, DataStackError,
};

pub fn evaluate(
    syntax_element: SyntaxElement,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    match syntax_element {
        SyntaxElement::Block { syntax_tree } => {
            data_stack.push(Value::Block(syntax_tree));
            Ok(())
        }
        SyntaxElement::Word(word) => evaluator_word(word, data_stack),
    }
}

fn evaluator_word(
    word: String,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    match word.as_str() {
        "true" => data_stack.push(true),
        "false" => data_stack.push(false),
        "not" => {
            let a = data_stack.pop_bool()?;
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
