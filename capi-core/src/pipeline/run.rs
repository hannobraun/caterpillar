use crate::{
    pipeline::stages::b_parser::parse,
    repr::eval::fragments::{FragmentId, Fragments},
};

use super::stages::{
    a_tokenizer::tokenize,
    b_parser::ParserError,
    c_simplifier::simplify,
    d_analyzer::{analyze, AnalyzerOutput},
    e_evaluator::evaluate,
};

pub fn run(
    code: &str,
    parent: Option<FragmentId>,
    fragments: &mut Fragments,
) -> Result<PipelineOutput, PipelineError> {
    let tokens = tokenize(code);
    let syntax_tree = parse(tokens)?;
    let syntax_tree = simplify(syntax_tree);
    let AnalyzerOutput { start } = analyze(syntax_tree, parent, fragments);
    let module = evaluate(start, &fragments);
    dbg!(module);

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
