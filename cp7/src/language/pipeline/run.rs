use crate::language::syntax::{FragmentId, Syntax};

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
    let ParserOutput { start, .. } =
        parse(&addressed_tokens, addressed_tokens.left_to_right(), syntax)?;

    Ok(PipelineOutput { start })
}

pub struct PipelineOutput {
    pub start: Option<FragmentId>,
}

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Failed to parse")]
    Parser(#[from] ParserError),
}
