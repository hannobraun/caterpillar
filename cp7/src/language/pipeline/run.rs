use crate::language::{
    syntax::{FragmentId, Syntax, SyntaxToTokens},
    tokens::Tokens,
};

use super::stages::{
    a_tokenizer::tokenize,
    b_addresser::address,
    d_parser::{parse, ParserError, ParserOutput},
};

pub fn run(
    code: &str,
    syntax: &mut Syntax,
) -> Result<PipelineOutput, PipelineError> {
    let tokens = tokenize(code);
    let addressed_tokens = address(tokens);
    let ParserOutput {
        start,
        syntax_to_tokens,
    } = parse(&addressed_tokens, addressed_tokens.left_to_right(), syntax)?;

    Ok(PipelineOutput {
        start,
        tokens: addressed_tokens,
        syntax_to_tokens,
    })
}

pub struct PipelineOutput {
    pub start: Option<FragmentId>,
    pub tokens: Tokens,
    pub syntax_to_tokens: SyntaxToTokens,
}

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Failed to parse")]
    Parser(#[from] ParserError),
}
