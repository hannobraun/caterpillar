use std::sync::mpsc::{Receiver, TryRecvError};

use crate::language::{
    pipeline::{self, PipelineError},
    syntax::Syntax,
};

use super::{
    evaluator::{Evaluator, EvaluatorError, EvaluatorState},
    interpreter::Interpreter,
    updater,
};

pub fn start(
    code: &str,
    updates: Receiver<String>,
) -> Result<(), RuntimeError> {
    let mut syntax = Syntax::new();

    let Some(start) = pipeline::run(code, &mut syntax)? else {
        return Ok(());
    };

    let mut interpreter = Interpreter {
        syntax,
        evaluator: Evaluator::new(start),
    };

    while let EvaluatorState::InProgress =
        interpreter.evaluator.step(&interpreter.syntax)?
    {
        match updates.try_recv() {
            Ok(code) => {
                interpreter.syntax.prepare_update();
                pipeline::run(&code, &mut interpreter.syntax)?;
                updater::update(
                    &interpreter.syntax,
                    &mut interpreter.evaluator,
                );
            }
            Err(TryRecvError::Empty) => {
                // nothing to do; just continue with the next evaluator step
            }
            Err(TryRecvError::Disconnected) => {
                return Err(RuntimeError::UpdatesDisconnected);
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
