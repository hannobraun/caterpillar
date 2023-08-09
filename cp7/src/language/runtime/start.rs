use std::sync::mpsc::{Receiver, TryRecvError};

use crate::language::{
    pipeline::{self, PipelineError},
    syntax::Syntax,
};

use super::{
    evaluator::{Evaluator, EvaluatorError, EvaluatorState},
    updater,
};

pub fn start(
    code: &str,
    updates: Receiver<String>,
) -> Result<(), RuntimeError> {
    let mut syntax = Syntax::new();

    let start = pipeline::run(code, &mut syntax)?;

    if let Some(start) = start {
        let mut evaluator = Evaluator::new(start);
        while let EvaluatorState::InProgress = evaluator.step(&syntax)? {
            match updates.try_recv() {
                Ok(code) => {
                    syntax.prepare_update();
                    pipeline::run(&code, &mut syntax)?;
                    updater::update(&syntax, &mut evaluator);
                }
                Err(TryRecvError::Empty) => {
                    // nothing to do; just continue with the next evaluator step
                }
                Err(TryRecvError::Disconnected) => {
                    return Err(RuntimeError::UpdatesDisconnected);
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Pipeline error")]
    Pipeline(#[from] PipelineError),

    #[error("Evaluator error")]
    Evaluator(#[from] EvaluatorError),

    #[error("Update channel is disconnected")]
    UpdatesDisconnected,
}
