use std::{
    path::Path,
    sync::mpsc::{Receiver, TryRecvError},
};

use crate::{
    pipeline::{self, PipelineError},
    runtime::updater,
    syntax::Syntax,
};

use super::evaluator::{Evaluator, EvaluatorError, EvaluatorState};

pub fn start(
    path: impl AsRef<Path>,
    updates: Receiver<()>,
) -> Result<(), RuntimeError> {
    let mut syntax = Syntax::new();

    let start = pipeline::run(path.as_ref(), &mut syntax)?;

    if let Some(start) = start {
        let mut evaluator = Evaluator::new(start);
        while let EvaluatorState::InProgress = evaluator.step(&syntax)? {
            match updates.try_recv() {
                Ok(()) => {
                    syntax.prepare_update();
                    pipeline::run(path.as_ref(), &mut syntax)?;
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
