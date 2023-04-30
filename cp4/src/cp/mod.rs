mod data_stack;
mod pipeline;

pub use self::{
    data_stack::{DataStack, DataStackError},
    pipeline::d_evaluator::EvaluatorError,
};

pub async fn execute(
    code: &str,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    let tokens = pipeline::a_tokenizer::tokenize(code.chars());
    pipeline::d_evaluator::evaluate(tokens, data_stack).await?;

    Ok(())
}
