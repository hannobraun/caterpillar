mod data_stack;
mod pipeline;

use std::{future, pin::pin};

use futures::stream;

pub use self::{
    data_stack::{DataStack, DataStackError},
    pipeline::d_evaluator::EvaluatorError,
};

pub async fn execute(
    code: &str,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    let mut code = pin!(stream::iter(code.chars()));
    let mut tokenizer = pipeline::a_tokenizer::Tokenizer::new();

    while let Some(token) = tokenizer.tokenize(code.as_mut()).await {
        pipeline::d_evaluator::evaluate(future::ready(token), data_stack)
            .await?;
    }

    Ok(())
}
