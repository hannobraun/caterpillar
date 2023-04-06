mod call_stack;
mod data_stack;
mod expressions;
mod functions;
mod keywords;
mod pipeline;
mod syntax;
mod tokens;

pub use self::{
    call_stack::CallStack,
    data_stack::{DataStack, Error as DataStackError},
    functions::Functions,
};

pub fn execute(
    code: impl IntoIterator<Item = char>,
) -> Result<(Functions, DataStack), Error> {
    let mut functions = Functions::new();
    let mut data_stack = DataStack::new();

    let tokens = pipeline::tokenize(code);
    let syntax_tree = pipeline::parse(tokens)?;
    let expressions = pipeline::analyze(syntax_tree, &mut functions);

    pipeline::evaluate(
        expressions,
        &functions,
        &mut CallStack,
        &mut data_stack,
    )?;

    Ok((functions, data_stack))
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
pub enum Error {
    #[error("Parser error: {0}")]
    Parser(#[from] pipeline::b_parser::Error),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] pipeline::d_evaluator::ErrorKind),
}
