mod data_stack;
mod evaluator;
mod parser;
mod tokenizer;

pub use self::{
    data_stack::DataStack, evaluator::evaluate, parser::parse,
    tokenizer::tokenize,
};

pub fn execute(code: &str) -> Result<DataStack, Error> {
    let mut data_stack = DataStack::new();

    let tokens = tokenize(code);
    let expressions = parse(tokens);
    evaluate(expressions, &mut data_stack)?;

    Ok(data_stack)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Evaluator error")]
    Evaluator(#[from] evaluator::Error),
}
