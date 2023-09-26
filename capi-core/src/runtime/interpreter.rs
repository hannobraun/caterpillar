use crate::{
    intrinsics,
    pipeline::{self, PipelineError, PipelineOutput},
    repr::eval::fragments::{Fragments, Replacement},
};

use super::{
    evaluator::{Evaluator, EvaluatorError, EvaluatorState},
    functions::NativeFunction,
};

#[derive(Debug)]
pub struct Interpreter {
    fragments: Fragments,
    evaluator: Evaluator,
}

impl Interpreter {
    pub fn new(code: &str) -> Result<Self, PipelineError> {
        let mut fragments = Fragments::new();
        let PipelineOutput { start } = pipeline::run(code, &mut fragments)?;

        let mut evaluator = Evaluator::new();
        evaluator.call_stack.push(start);

        for (name, intrinsic) in intrinsics::list() {
            evaluator.functions.register_native(name, intrinsic)
        }

        Ok(Interpreter {
            fragments,
            evaluator,
        })
    }

    pub fn register_platform(
        &mut self,
        functions: impl IntoIterator<Item = (&'static str, NativeFunction)>,
    ) -> &mut Self {
        self.evaluator.functions.register_platform(functions);
        self
    }

    pub fn step(&mut self) -> Result<EvaluatorState, EvaluatorError> {
        self.evaluator.step(&self.fragments)
    }

    pub fn update(&mut self, code: &str) -> Result<(), PipelineError> {
        let PipelineOutput { start } =
            pipeline::run(code, &mut self.fragments)?;

        for Replacement { old, new } in self.fragments.take_replacements() {
            self.evaluator.call_stack.replace(old, new);
            self.evaluator.data_stack.replace(old, new);
            self.evaluator.functions.replace(old, new, &self.fragments);
        }

        if self.evaluator.state().finished() {
            self.evaluator.call_stack.push(start);
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
    use crate::{
        runtime::interpreter::Interpreter, value, DataStackResult, Evaluator,
        NativeFunction,
    };

    // Make sure all updates happen in the middle of their respective context,
    // not the beginning. This is the more complex case, and leads to the test
    // exercising more of the relevant machinery.

    #[test]
    fn update_to_named_function() -> anyhow::Result<()> {
        let original = ":f { nop 1 ping f } fn f";
        let updated = ":f { nop 2 ping f } fn f";

        let mut interpreter = interpreter(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_to_identical_functions() -> anyhow::Result<()> {
        let original = "
            :loop { f loop } fn
            :f { nop 1 ping } fn
            :g { nop 1 ping } fn
            loop";
        let updated = "
            :loop { g loop } fn
            :f { nop 2 ping } fn
            :g { nop 1 ping } fn
            loop";

        let mut interpreter = interpreter(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }

    #[test]
    fn update_that_reverts_back_to_an_earlier_version() -> anyhow::Result<()> {
        let original = ":f { nop 1 ping f } fn f";
        let updated = ":f { nop 2 ping f } fn f";

        let mut interpreter = interpreter(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }

    #[test]
    fn update_to_block() -> anyhow::Result<()> {
        let original = "{ nop 1 ping } clone eval eval";
        let updated = "{ nop 2 ping } clone eval eval";

        let mut interpreter = interpreter(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_to_identical_blocks_at_end_of_context() -> anyhow::Result<()> {
        let original = "
            :f { { nop 2 ping } } fn
            :g { { nop 2 ping } } fn
            1 ping
            f eval
            g eval";
        let updated = "
            :f { { nop 2 ping } } fn
            :g { { nop 3 ping } } fn
            1 ping
            f eval
            g eval";

        let mut interpreter = interpreter(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;
        interpreter.wait_for_ping_on_channel(3)?;

        Ok(())
    }

    #[test]
    fn update_function_caller() -> anyhow::Result<()> {
        let original = "
            :f { nop 1 ping } fn
            f
            1 ping";
        let updated = "
            :f { nop 1 ping } fn
            f
            2 ping";

        let mut interpreter = interpreter(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_renamed_function() -> anyhow::Result<()> {
        let original = ":f { nop 1 ping f } fn f";
        let updated = ":g { nop 1 ping g } fn g";

        let mut interpreter = interpreter(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }

    fn interpreter(code: &str) -> anyhow::Result<Interpreter> {
        let mut interpreter = Interpreter::new(code)?;
        interpreter.register_platform([("ping", ping as NativeFunction)]);
        Ok(interpreter)
    }

    pub fn ping(evaluator: &mut Evaluator) -> DataStackResult<()> {
        let (channel, _) =
            evaluator.data_stack.pop_specific::<value::Number>()?;
        *evaluator.context.channels.entry(channel.0).or_insert(0) += 1;
        Ok(())
    }
}
