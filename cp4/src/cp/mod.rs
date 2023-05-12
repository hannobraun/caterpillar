mod data_stack;
mod functions;
mod pipeline;
mod syntax;

pub use self::{
    data_stack::{DataStack, DataStackError},
    pipeline::d_evaluator::EvaluatorError,
};

pub async fn execute(
    code: pipeline::a_tokenizer::Chars,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    let mut functions = functions::Functions::new();

    let tokenizer = pipeline::a_tokenizer::Tokenizer::new(code);
    let parser = pipeline::b_parser::Parser::new(tokenizer);
    let mut evaluator = pipeline::d_evaluator::Evaluator::new(Box::new(parser));

    match evaluator.evaluate(data_stack, &mut functions).await {
        Ok(()) => {}
        Err(err) if err.is_no_more_chars() => {}
        Err(err) => return Err(err),
    }

    Ok(())
}
