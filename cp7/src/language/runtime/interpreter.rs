use crate::language::{
    intrinsics,
    pipeline::{self, PipelineError, PipelineOutput},
    repr::syntax::Fragments,
};

use super::{
    evaluator::{Evaluator, EvaluatorError, EvaluatorState},
    functions::Intrinsic,
    updater,
};

#[derive(Debug)]
pub struct Interpreter {
    pub syntax: Fragments,
    pub evaluator: Evaluator,
}

impl Interpreter {
    pub fn new(code: &str) -> Result<Self, PipelineError> {
        let mut syntax = Fragments::new();
        let PipelineOutput { start } = pipeline::run(code, &mut syntax)?;

        let mut evaluator = Evaluator::new();
        if let Some(start) = start {
            evaluator.call_stack.push(start);
        }

        let intrinsics = [
            ("+", intrinsics::add as Intrinsic),
            ("clone", intrinsics::clone),
            ("delay_ms", intrinsics::delay_ms),
            ("print_line", intrinsics::print_line),
            ("fn", intrinsics::fn_),
        ];

        for (name, intrinsic) in intrinsics {
            evaluator.functions.register_intrinsic(name, intrinsic)
        }

        Ok(Interpreter { syntax, evaluator })
    }

    pub fn step(&mut self) -> Result<EvaluatorState, EvaluatorError> {
        self.evaluator.step(&self.syntax)
    }

    pub fn update(&mut self, code: &str) -> Result<(), PipelineError> {
        pipeline::run(code, &mut self.syntax)?;
        updater::update(&mut self.syntax, &mut self.evaluator);

        Ok(())
    }
}
