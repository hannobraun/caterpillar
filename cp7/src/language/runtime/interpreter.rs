use crate::language::{
    intrinsics,
    pipeline::{self, PipelineError, PipelineOutput},
    repr::eval::fragments::{Fragments, Replacement},
};

use super::{
    evaluator::{Evaluator, EvaluatorError, EvaluatorState},
    functions::Intrinsic,
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
        evaluator.call_stack.push(start);

        let intrinsics = [
            ("+", intrinsics::add as Intrinsic),
            ("clone", intrinsics::clone),
            ("delay_ms", intrinsics::delay_ms),
            ("fn", intrinsics::fn_),
            ("over", intrinsics::over),
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

        for Replacement { old, new } in self.fragments.take_replacements() {
            self.evaluator.functions.replace(old, new);
        }

        Ok(())
    }

    #[cfg(test)]
    pub fn wait_for_ping_on_channel(
        &mut self,
        channel: i64,
    ) -> Result<(), EvaluatorError> {
        self.evaluator.context.channels.clear();

        let mut num_steps = 0;

        loop {
            if self.evaluator.context.channels.contains_key(&channel)
                && self.evaluator.context.channels[&channel] == 1
            {
                break;
            }

            self.step()?;
            num_steps += 1;

            if num_steps == 1024 {
                panic!("Waiting for ping on channel {channel} took too long");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::language::runtime::interpreter::Interpreter;

    // Make sure all updates happen in the middle of their respective context,
    // not the beginning. This is the more complex case, and leads to the test
    // exercising more of the relevant machinery.

    #[test]
    fn update_to_named_function() -> anyhow::Result<()> {
        let original = ":f { 1 1 ping f } fn f";
        let updated = ":f { 1 2 ping f } fn f";

        let mut interpreter = Interpreter::new(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_to_identical_functions() -> anyhow::Result<()> {
        let original = "
            :loop { f loop } fn
            :f { 1 ping } fn
            :g { 1 ping } fn
            loop";
        let updated = "
            :loop { g loop } fn
            :f { 2 ping } fn
            :g { 1 ping } fn
            loop";

        let mut interpreter = Interpreter::new(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }

    #[test]
    fn update_that_reverts_back_to_an_earlier_version() -> anyhow::Result<()> {
        let original = ":f { 1 1 ping f } fn f";
        let updated = ":f { 1 2 ping f } fn f";

        let mut interpreter = Interpreter::new(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }
}
