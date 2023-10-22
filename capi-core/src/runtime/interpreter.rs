use crate::{
    pipeline::{self, PipelineError, PipelineOutput},
    repr::eval::fragments::{Fragments, Replacement},
    PlatformFunction,
};

use super::evaluator::{Evaluator, EvaluatorError, RuntimeState};

#[derive(Debug)]
pub struct Interpreter<C> {
    fragments: Fragments,
    evaluator: Evaluator<C>,
    state: RuntimeState,
}

impl<C> Interpreter<C> {
    pub fn new(code: &str) -> Result<Self, PipelineError> {
        let mut fragments = Fragments::new();
        let PipelineOutput { start } = pipeline::run(code, &mut fragments)?;

        let mut evaluator = Evaluator::default();
        evaluator.call_stack.push(start);

        Ok(Interpreter {
            fragments,
            evaluator,
            state: RuntimeState::Running,
        })
    }

    pub fn register_platform(
        &mut self,
        functions: impl IntoIterator<Item = (&'static str, PlatformFunction<C>)>,
    ) {
        self.evaluator.namespace.register_platform(functions);
    }

    pub fn step(
        &mut self,
        platform_context: &mut C,
    ) -> Result<RuntimeState, EvaluatorError> {
        self.state = self.evaluator.step(&self.fragments, platform_context)?;
        Ok(self.state)
    }

    pub fn update(&mut self, code: &str) -> Result<(), PipelineError> {
        let PipelineOutput { start } =
            pipeline::run(code, &mut self.fragments)?;

        for Replacement { old, new } in self.fragments.take_replacements() {
            self.evaluator.call_stack.replace(old, new);
            self.evaluator.data_stack.replace(old, new);
            self.evaluator.namespace.replace(old, new, &self.fragments);
        }

        // If the program has finished running, restart it in response to this
        // update.
        if self.state.finished() {
            self.evaluator.call_stack.push(start);
        }

        Ok(())
    }
}

#[cfg(test)]
impl Interpreter<()> {
    pub fn run_tests(&mut self) -> anyhow::Result<()> {
        use crate::value;
        use anyhow::bail;

        while !self.step(&mut ())?.finished() {}

        for function in self.evaluator.namespace.user_defined().functions() {
            let mut evaluator = Evaluator::default();
            evaluator.call_stack.push(function.body.start);

            while !evaluator.step(&self.fragments, &mut ())?.finished() {}

            let (result, _) =
                evaluator.data_stack.pop_specific::<value::Bool>()?;

            if !evaluator.data_stack.is_empty() {
                bail!("Test returned more than one `bool`");
            }
            if !result.0 {
                bail!("Test returned `false`");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        pipeline::PipelineError, runtime::evaluator::EvaluatorError, value,
        DataStackResult, FunctionState, PlatformFunction, RuntimeContext,
    };

    // Make sure all updates happen in the middle of their respective context,
    // not the beginning. This is the more complex case, and leads to the test
    // exercising more of the relevant machinery.

    #[test]
    fn update_to_named_function() -> anyhow::Result<()> {
        let original = ":f { nop 1 ping f } fn f";
        let updated = ":f { nop 2 ping f } fn f";

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
            :f { nop 1 ping } fn
            :g { nop 1 ping } fn
            loop";
        let updated = "
            :loop { g loop } fn
            :f { nop 2 ping } fn
            :g { nop 1 ping } fn
            loop";

        let mut interpreter = Interpreter::new(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }

    #[test]
    fn update_that_reverts_back_to_an_earlier_version() -> anyhow::Result<()> {
        let original = ":f { nop 1 ping f } fn f";
        let updated = ":f { nop 2 ping f } fn f";

        let mut interpreter = Interpreter::new(original)?;
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

        let mut interpreter = Interpreter::new(original)?;
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

        let mut interpreter = Interpreter::new(original)?;
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

        let mut interpreter = Interpreter::new(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_renamed_function() -> anyhow::Result<()> {
        let original = ":f { nop 1 ping f } fn f";
        let updated = ":g { nop 1 ping g } fn g";

        let mut interpreter = Interpreter::new(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }

    struct Interpreter {
        inner: crate::Interpreter<PlatformContext>,
        platform_context: PlatformContext,
    }

    impl Interpreter {
        pub fn new(code: &str) -> anyhow::Result<Self> {
            let mut inner = crate::Interpreter::new(code)?;

            inner.register_platform([(
                "ping",
                ping as PlatformFunction<PlatformContext>,
            )]);

            Ok(Self {
                inner,
                platform_context: PlatformContext::default(),
            })
        }

        pub fn update(&mut self, code: &str) -> Result<(), PipelineError> {
            self.inner.update(code)
        }

        pub fn wait_for_ping_on_channel(
            &mut self,
            channel: i64,
        ) -> Result<(), EvaluatorError> {
            self.platform_context.channels.clear();

            let mut num_steps = 0;

            loop {
                if self.platform_context.channels.contains_key(&channel)
                    && self.platform_context.channels[&channel] == 1
                {
                    break;
                }

                self.inner.step(&mut self.platform_context)?;
                num_steps += 1;

                if num_steps == 1024 {
                    panic!(
                        "Waiting for ping on channel {channel} took too long"
                    );
                }
            }

            Ok(())
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct PlatformContext {
        pub channels: HashMap<i64, i64>,
    }

    pub fn ping(
        runtime_context: RuntimeContext,
        platform_context: &mut PlatformContext,
    ) -> DataStackResult<FunctionState> {
        let (channel, _) =
            runtime_context.data_stack.pop_specific::<value::Number>()?;
        *platform_context.channels.entry(channel.0).or_insert(0) += 1;
        Ok(FunctionState::Done)
    }
}
