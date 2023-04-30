mod data_stack;
mod pipeline;

use std::pin::pin;

use futures::stream;

pub use self::{
    data_stack::{DataStack, DataStackError},
    pipeline::d_evaluator::EvaluatorError,
};

pub async fn execute(
    code: &str,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    let tokens = pipeline::a_tokenizer::tokenize(code.chars());
    let tokens = pin!(stream::iter(tokens));
    pipeline::d_evaluator::evaluate(tokens, data_stack).await?;

    Ok(())
}
