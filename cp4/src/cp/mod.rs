mod data_stack;
mod pipeline;

use std::future;

pub use self::{
    data_stack::{DataStack, DataStackError},
    pipeline::d_evaluator::EvaluatorError,
};

pub async fn execute(
    code: &str,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    let code = code.chars();
    let mut tokenizer = pipeline::a_tokenizer::Tokenizer::new();

    let tokens = tokenizer.tokenize(code);
    for token in tokens {
        pipeline::d_evaluator::evaluate(future::ready(token), data_stack)
            .await?;
    }

    Ok(())
}
