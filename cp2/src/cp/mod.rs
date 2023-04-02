mod analyzer;
mod call_stack;
mod data_stack;
mod evaluator;
mod parser;
mod pipeline;

pub use self::{
    analyzer::{analyze, Functions},
    call_stack::CallStack,
    data_stack::{DataStack, Error as DataStackError},
    evaluator::evaluate,
    parser::{parse, SyntaxTree},
    pipeline::a_tokenizer::tokenize,
};

pub fn execute(code: &str) -> Result<(Functions, DataStack), Error> {
    let mut functions = Functions::new();
    let mut data_stack = DataStack::new();

    let tokens = tokenize(code);
    let syntax_tree = parse(tokens)?;
    let expressions = analyze(syntax_tree, &mut functions);

    evaluate(expressions, &functions, &mut CallStack, &mut data_stack)?;

    Ok((functions, data_stack))
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
pub enum Error {
    #[error("Parser error: {0}")]
    Parser(#[from] parser::Error),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] evaluator::ErrorKind),
}
