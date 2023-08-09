use std::{
    io,
    path::Path,
    sync::mpsc::{Receiver, TryRecvError},
};

use crate::{
    language::{
        pipeline::{self, PipelineError},
        syntax::Syntax,
    },
    loader::load::load,
};

use super::{
    evaluator::{Evaluator, EvaluatorError, EvaluatorState},
    updater,
};

pub fn start(
    path: impl AsRef<Path>,
    updates: Receiver<String>,
) -> Result<(), RuntimeError> {
    let mut syntax = Syntax::new();

    let code = load(path.as_ref())?;
    let start = pipeline::run(&code, &mut syntax)?;

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
    #[error("Failed to load code from file")]
    Loader(#[from] io::Error),

    #[error("Pipeline error")]
    Pipeline(#[from] PipelineError),

    #[error("Evaluator error")]
    Evaluator(#[from] EvaluatorError),

    #[error("Update channel is disconnected")]
    UpdatesDisconnected,
}
