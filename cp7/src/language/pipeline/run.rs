use crate::language::{
    syntax::{Syntax, SyntaxHandle},
    tokens::Tokens,
};

use super::stages::{
    a_tokenizer::tokenize,
    b_addresser::address,
    c_parser::{parse, ParserError},
};

pub fn run(
    code: &str,
    syntax: &mut Syntax,
) -> Result<(Option<SyntaxHandle>, Tokens), PipelineError> {
    let tokens = tokenize(code);
    let tokens = address(tokens);
    let start = parse(tokens.iter(), syntax)?;

    Ok((start, tokens))
}

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Failed to parse")]
    Parser(#[from] ParserError),
}
