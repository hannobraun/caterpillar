use crate::{
    pipeline::stages::b_parser::parse,
    repr::eval::fragments::{FragmentId, Fragments},
};

use super::stages::{
    a_tokenizer::tokenize,
    b_parser::ParserError,
    d_analyzer::{analyze, AnalyzerOutput},
};

pub fn run(
    code: &str,
    parent: Option<FragmentId>,
    fragments: &mut Fragments,
) -> Result<PipelineOutput, PipelineError> {
    let tokens = tokenize(code);
    let syntax_tree = parse(tokens)?;
    let AnalyzerOutput { start } = analyze(syntax_tree, parent, fragments);

    Ok(PipelineOutput { start })
}

pub struct PipelineOutput {
    pub start: FragmentId,
}

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Failed to parse")]
    Parser(#[from] ParserError),
}
