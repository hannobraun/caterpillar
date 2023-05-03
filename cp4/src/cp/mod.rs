mod data_stack;
mod pipeline;

pub use self::{
    data_stack::{DataStack, DataStackError},
    pipeline::d_evaluator::EvaluatorError,
};

pub async fn execute(
    code: pipeline::a_tokenizer::Chars,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    let mut tokenizer = pipeline::a_tokenizer::Tokenizer::new(code);
    let mut evaluator = pipeline::d_evaluator::Evaluator;

    while let Some(token) = tokenizer.next_token().await {
        evaluator
            .evaluate(std::future::ready(token), data_stack)
            .await?;
    }

    Ok(())
}
