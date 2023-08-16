use crate::language::{
    syntax::{Syntax, SyntaxHandle, TokensToSyntax},
    tokens::Tokens,
};

use super::stages::{
    a_tokenizer::tokenize,
    b_addresser::address,
    c_parser::{parse, ParserError, ParserOutput},
};

pub fn run(
    code: &str,
    syntax: &mut Syntax,
) -> Result<PipelineOutput, PipelineError> {
    let tokens = tokenize(code);
    let tokens = address(tokens);
    let ParserOutput {
        start,
        tokens_to_syntax,
    } = parse(tokens.iter(), syntax)?;

    Ok(PipelineOutput {
        start,
        tokens,
        tokens_to_syntax,
    })
}

pub struct PipelineOutput {
    pub start: Option<SyntaxHandle>,
    pub tokens: Tokens,
    pub tokens_to_syntax: TokensToSyntax,
}

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Failed to parse")]
    Parser(#[from] ParserError),
}
