use crate::{
    pipeline::{self, PipelineError, PipelineOutput},
    repr::eval::fragments::{Fragments, Replacement},
    value, PlatformFunction,
};

use super::{
    data_stack::{DataStack, DataStackError},
    evaluator::{Evaluator, EvaluatorError, RuntimeState},
};

#[derive(Debug)]
pub struct Interpreter<C> {
    fragments: Fragments,
    evaluator: Evaluator<C>,
    state: RuntimeState,
}

impl<C> Interpreter<C> {
    pub fn new() -> Result<Self, PipelineError> {
        let fragments = Fragments::new();
        let evaluator = Evaluator::default();

        Ok(Interpreter {
            fragments,
            evaluator,
            state: RuntimeState::Finished,
        })
    }

    pub fn register_platform(
        &mut self,
        functions: impl IntoIterator<Item = (PlatformFunction<C>, &'static str)>,
    ) {
        self.evaluator.global_namespace.register_platform(functions);
    }

    pub fn step(
        &mut self,
        platform_context: &mut C,
    ) -> Result<RuntimeState, EvaluatorError> {
        self.state =
            self.evaluator.step(&mut self.fragments, platform_context)?;
        Ok(self.state)
    }

    pub fn update(&mut self, code: &str) -> Result<(), PipelineError> {
        let parent = None;
        let PipelineOutput { start } =
            pipeline::run(code, parent, &mut self.fragments)?;

        for Replacement { old, new } in self.fragments.take_replacements() {
            self.evaluator.call_stack.replace(old, new);
            self.evaluator.data_stack.replace(old, new);
            self.evaluator
                .global_namespace
                .replace(old, new, &self.fragments);
        }

        // If the program has finished running, restart it in response to this
        // update.
        if self.state.finished() {
            self.evaluator.call_stack.push(start);
        }

        Ok(())
    }

    pub fn run_tests(
        &mut self,
        platform_context: &mut C,
    ) -> Result<(), TestError> {
        while !self.step(platform_context)?.finished() {}

        let tests = self
            .evaluator
            .global_namespace
            .user_defined()
            .tests()
            .cloned()
            .collect::<Vec<_>>();

        if !self.evaluator.data_stack.is_empty() {
            // This happens easily, if you do most of the work of defining a
            // test, but then forgot to actually write `test` at the end.
            // Without this error, it would result in dead code that's never
            // actually run.
            return Err(TestError::DataStackNotEmptyAfterScriptEvaluation {
                data_stack: self.evaluator.data_stack.clone(),
            });
        }

        for function in tests {
            print!("Running test `{}`...", function.name.value);

            // We don't need to worry about any call stack contents from the
            // initial module evaluation, or the evaluation of the previous
            // test, interfering with the evaluation of the next test. When
            // evaluation is finished then, by definition, the call stack is
            // empty.
            self.evaluator.call_stack.push(function.body.start);
            self.evaluator.data_stack.clear();

            while !self
                .evaluator
                .step(&mut self.fragments, platform_context)?
                .finished()
            {}

            let (result, _) =
                self.evaluator.data_stack.pop_specific::<value::Bool>()?;

            if !self.evaluator.data_stack.is_empty() {
                return Err(TestError::DataStackNotEmptyAfterTestRun {
                    data_stack: self.evaluator.data_stack.clone(),
                });
            }
            if !result.0 {
                return Err(TestError::TestFailed);
            }

            println!(" PASS");
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TestError {
    #[error(transparent)]
    Evaluator(#[from] EvaluatorError),

    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error(
        "Data stack not empty after evaluating test definitions: {data_stack}"
    )]
    DataStackNotEmptyAfterScriptEvaluation { data_stack: DataStack },

    #[error("Expected test to return one `bool`; left on stack: {data_stack}")]
    DataStackNotEmptyAfterTestRun { data_stack: DataStack },

    #[error("Test returned `false`")]
    TestFailed,
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
            let mut inner = crate::Interpreter::new()?;
            inner.update(code)?;

            inner.register_platform([(
                ping as PlatformFunction<PlatformContext>,
                "ping",
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
