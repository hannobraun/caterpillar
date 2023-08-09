use crate::language::{
    pipeline::{self, PipelineError},
    syntax::Syntax,
};

use super::{
    evaluator::{Evaluator, EvaluatorError, EvaluatorState},
    updater,
};

pub struct Interpreter {
    pub syntax: Syntax,
    pub evaluator: Evaluator,
}

impl Interpreter {
    pub fn new(code: &str) -> Result<Option<Self>, PipelineError> {
        let mut syntax = Syntax::new();
        let start = pipeline::run(code, &mut syntax)?;

        let mut evaluator = Evaluator::new();
        if let Some(start) = start {
            evaluator.call_stack.push(start);
        }

        Ok(Some(Interpreter { syntax, evaluator }))
    }

    pub fn step(&mut self) -> Result<EvaluatorState, EvaluatorError> {
        self.evaluator.step(&self.syntax)
    }

    pub fn update(&mut self, code: &str) -> Result<(), PipelineError> {
        self.syntax.prepare_update();
        pipeline::run(code, &mut self.syntax)?;
        updater::update(&self.syntax, &mut self.evaluator);

        Ok(())
    }
}
