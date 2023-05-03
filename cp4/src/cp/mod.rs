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
    let code = pin!(stream::iter(code.chars()));
    let mut tokenizer = pipeline::a_tokenizer::Tokenizer::new();

    let mut tokens = tokenizer.tokenize(code).await;
    while let Some(token) = tokens.next().await {
        pipeline::d_evaluator::evaluate(future::ready(token), data_stack)
            .await?;
    }

    Ok(())
}
