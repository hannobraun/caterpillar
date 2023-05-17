mod chars;
mod data_stack;
mod functions;
mod pipeline;
mod syntax;

pub use self::{
    chars::Chars,
    data_stack::{DataStack, DataStackError},
    functions::Functions,
    pipeline::d_evaluator::{Evaluator, EvaluatorError},
};

pub async fn execute(
    code: Chars,
    data_stack: &mut DataStack,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), EvaluatorError> {
    let tokenizer = pipeline::a_tokenizer::Tokenizer::new(code);
    let parser = pipeline::b_parser::Parser::new(tokenizer);
    let mut evaluator = Evaluator::new(Box::new(parser));

    match evaluator.evaluate(data_stack, functions, tests).await {
        Ok(()) => {}
        Err(err) if err.is_no_more_chars() => {}
        Err(err) => return Err(err),
    }

    Ok(())
}
