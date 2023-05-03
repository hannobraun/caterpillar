mod data_stack;
mod pipeline;

use std::{future, pin::pin};

use futures::{stream, StreamExt};

pub use self::{
    data_stack::{DataStack, DataStackError},
    pipeline::d_evaluator::EvaluatorError,
};

pub async fn execute(
    code: &str,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    let tokens = pipeline::a_tokenizer::tokenize(code.chars());
    let mut tokens = pin!(stream::iter(tokens));
    while let Some(token) = tokens.next().await {
        pipeline::d_evaluator::evaluate(future::ready(token), data_stack)
            .await?;
    }

    Ok(())
}
