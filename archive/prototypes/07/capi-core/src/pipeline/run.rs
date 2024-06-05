use crate::{
    pipeline::stages::b_parser::parse,
    repr::eval::fragments::{FragmentId, Fragments},
    runtime::evaluator::EvaluatorError,
};

use super::{
    module::Module,
    scripts::Scripts,
    stages::{
        a_tokenizer::tokenize, b_parser::ParserError, c_simplifier::simplify,
        d_analyzer::analyze, e_evaluator::evaluate,
    },
};

pub fn run(
    code: &str,
    parent: Option<FragmentId>,
    fragments: &mut Fragments,
    scripts: &Scripts,
) -> Result<PipelineOutput, PipelineError> {
    let tokens = tokenize(code);
    let syntax_tree = parse(tokens)?;
    let syntax_tree = simplify(syntax_tree);
    let start = analyze(syntax_tree, parent, fragments);
    let module = evaluate(start, fragments, scripts)?;

    Ok(PipelineOutput { start, module })
}

pub struct PipelineOutput {
    pub start: FragmentId,
    pub module: Module,
}

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Failed to parse")]
    Parser(#[from] ParserError),

    #[error("Failed to evaluate top-level context")]
    Evaluator(#[from] EvaluatorError<()>),
}
