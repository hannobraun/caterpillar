mod call_stack;
mod data_stack;
mod expressions;
mod functions;
mod pipeline;
mod syntax;
mod tokens;

pub use self::{
    call_stack::CallStack,
    data_stack::{DataStack, Error as DataStackError},
    functions::Functions,
    pipeline::{
        a_tokenizer::tokenize, b_parser::parse, c_analyzer::analyze,
        d_evaluator::evaluate,
    },
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
    Parser(#[from] pipeline::b_parser::Error),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] pipeline::d_evaluator::ErrorKind),
}
