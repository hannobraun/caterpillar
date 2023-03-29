mod call_stack;
mod data_stack;
mod evaluator;
mod parser;
mod tokenizer;

pub use self::{
    call_stack::CallStack,
    data_stack::{DataStack, Error as DataStackError},
    evaluator::evaluate,
    parser::{parse, SyntaxTree},
    tokenizer::tokenize,
};

pub fn execute(code: &str) -> Result<DataStack, Error> {
    let mut data_stack = DataStack::new();

    let tokens = tokenize(code);
    let expressions = parse(tokens)?;
    evaluate(expressions, &mut CallStack, &mut data_stack)?;

    Ok(data_stack)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parser error: {0}")]
    Parser(#[from] parser::Error),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] evaluator::Error),
}
