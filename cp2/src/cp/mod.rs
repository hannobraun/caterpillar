mod call_stack;
mod data_stack;
mod evaluator;
mod parser;
mod semantic_analyzer;
mod tokenizer;

pub use self::{
    call_stack::CallStack,
    data_stack::{DataStack, Error as DataStackError},
    evaluator::evaluate,
    parser::{parse, Functions, SyntaxTree},
    semantic_analyzer::analyze,
    tokenizer::tokenize,
};

// It doesn't really matter what the name of the root function is, as long as
// it's no parsable as an identifier. If it were, the user could define a
// function to shadow this one.
const ROOT_FN: &str = ":root";

pub fn execute(code: &str) -> Result<DataStack, Error> {
    let mut functions = Functions::new();
    let mut data_stack = DataStack::new();

    let tokens = tokenize(code);
    let syntax_tree = parse(tokens, &mut functions)?;
    let syntax_tree = analyze(syntax_tree);
    evaluate(syntax_tree, &functions, &mut CallStack, &mut data_stack)?;

    Ok(data_stack)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parser error: {0}")]
    Parser(#[from] parser::Error),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] evaluator::ErrorKind),
}
