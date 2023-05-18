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
        pipeline::d_evaluator::evaluate(word, data_stack)?;
    }

    Ok(())
}
