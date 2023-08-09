use crate::language::syntax::{Syntax, SyntaxHandle};

use super::stages::{
    a_tokenizer::tokenize,
    b_parser::{parse, ParserError},
};

pub fn run(
    code: &str,
    syntax: &mut Syntax,
) -> Result<Option<SyntaxHandle>, PipelineError> {
    let tokens = tokenize(code);
    let start = parse(tokens, syntax)?;

    Ok(start)
}

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Failed to parse")]
    Parser(#[from] ParserError),
}
