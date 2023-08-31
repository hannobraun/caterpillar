use crate::language::repr::eval::fragments::{FragmentId, Fragments};

use super::stages::{
    a_tokenizer::tokenize,
    b_parser::ParserError,
    c_analyzer::{analyze, AnalyzerOutput},
};

pub fn run(
    code: &str,
    fragments: &mut Fragments,
) -> Result<PipelineOutput, PipelineError> {
    let tokens = tokenize(code);
    let AnalyzerOutput { start } = analyze(tokens, fragments)?;

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
