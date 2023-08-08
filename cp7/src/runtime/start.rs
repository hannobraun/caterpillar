use std::path::Path;

use crate::{
    pipeline::{self, PipelineError},
    syntax::Syntax,
};

use super::evaluator::{Evaluator, EvaluatorError, EvaluatorState};

pub fn start(path: impl AsRef<Path>) -> Result<(), RuntimeError> {
    let mut syntax = Syntax::new();

    let start = pipeline::run(path.as_ref(), &mut syntax)?;

    if let Some(start) = start {
        let mut evaluator = Evaluator::new(start);
        while let EvaluatorState::InProgress = evaluator.step(&syntax)? {}
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Pipeline error")]
    Pipeline(#[from] PipelineError),

    #[error("Evaluator error")]
    Error(#[from] EvaluatorError),
}
