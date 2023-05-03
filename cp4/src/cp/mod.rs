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
    let tokenizer = pipeline::a_tokenizer::Tokenizer::new(code);
    let parser = pipeline::b_parser::Parser::new(tokenizer);
    let mut evaluator = pipeline::d_evaluator::Evaluator::new(parser);

    evaluator.evaluate(data_stack).await?;

    Ok(())
}
