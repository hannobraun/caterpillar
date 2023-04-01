mod analyzer;
mod call_stack;
mod data_stack;
mod evaluator;
mod parser;
mod tokenizer;

pub use self::{
    analyzer::{analyze, Functions},
    call_stack::CallStack,
    data_stack::{DataStack, Error as DataStackError},
    evaluator::evaluate,
    parser::{parse, SyntaxTree},
    tokenizer::tokenize,
};

pub fn execute(code: &str) -> Result<DataStack, Error> {
    let mut functions = Functions::new();
    let mut data_stack = DataStack::new();

    let tokens = tokenize(code);
    let syntax_tree = parse(tokens)?;
    let expressions = analyze(syntax_tree, &mut functions);

    evaluate(expressions, &functions, &mut CallStack, &mut data_stack)?;

    Ok(data_stack)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parser error: {0}")]
    Parser(#[from] parser::Error),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] evaluator::ErrorKind),
}
