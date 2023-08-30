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
    let _ = address(tokens.clone());
    let ParserOutput { start } = parse(tokens.iter(), syntax)?;

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
