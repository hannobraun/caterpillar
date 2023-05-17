mod data_stack;
mod pipeline;

pub use self::{
    data_stack::{DataStack, DataStackError},
    pipeline::d_evaluator::EvaluatorError,
};

pub fn execute(
    code: &str,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    for word in code.split_whitespace() {
        match word {
            "true" => data_stack.push(true),
            "false" => data_stack.push(false),
            "not" => {
                let a = data_stack.pop()?;
                let x = !a;
                data_stack.push(x);
            }
            _ => return Err(EvaluatorError::UnknownWord(word.into())),
        }
    }

    Ok(())
}
