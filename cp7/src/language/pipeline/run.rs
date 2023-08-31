use crate::language::repr::syntax::{FragmentId, Syntax};

use super::stages::{
    a_tokenizer::tokenize,
    c_analyzer::{analyze, ParserError, ParserOutput},
};

pub fn run(
    code: &str,
    syntax: &mut Syntax,
) -> Result<PipelineOutput, PipelineError> {
    let tokens = tokenize(code);
    let ParserOutput { start } = analyze(tokens, syntax)?;

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
