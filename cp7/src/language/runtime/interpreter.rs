use crate::language::{
    intrinsics,
    pipeline::{self, PipelineError, PipelineOutput},
    repr::eval::fragments::Fragments,
};

use super::{
    evaluator::{Evaluator, EvaluatorError, EvaluatorState},
    functions::Intrinsic,
    updater,
};

#[derive(Debug)]
pub struct Interpreter {
    pub fragments: Fragments,
    pub evaluator: Evaluator,
}

impl Interpreter {
    pub fn new(code: &str) -> Result<Self, PipelineError> {
        let mut fragments = Fragments::new();
        let PipelineOutput { start } = pipeline::run(code, &mut fragments)?;

        let mut evaluator = Evaluator::new();
        if let Some(start) = start {
            evaluator.call_stack.push(start);
        }

        let intrinsics = [
            ("+", intrinsics::add as Intrinsic),
            ("clone", intrinsics::clone),
            ("delay_ms", intrinsics::delay_ms),
            ("fn", intrinsics::fn_),
            ("ping", intrinsics::ping),
            ("print_line", intrinsics::print_line),
        ];

        for (name, intrinsic) in intrinsics {
            evaluator.functions.register_intrinsic(name, intrinsic)
        }

        Ok(Interpreter {
            fragments,
            evaluator,
        })
    }

    pub fn step(&mut self) -> Result<EvaluatorState, EvaluatorError> {
        self.evaluator.step(&self.fragments)
    }

    pub fn update(&mut self, code: &str) -> Result<(), PipelineError> {
        pipeline::run(code, &mut self.fragments)?;
        updater::update(&mut self.fragments, &mut self.evaluator);

        Ok(())
    }

    #[cfg(test)]
    pub fn wait_for_ping_on_channel(
        &mut self,
        channel: i64,
    ) -> Result<(), EvaluatorError> {
        loop {
            if self.evaluator.context.channels.contains_key(&channel)
                && self.evaluator.context.channels[&channel] == 1
            {
                break;
            }
            self.step()?;
        }

        Ok(())
    }
}
