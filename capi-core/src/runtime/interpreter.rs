use crate::{
    pipeline::{self, module::Module, PipelineError, PipelineOutput, Scripts},
    repr::eval::fragments::{FragmentId, Fragments, Replacement},
    value, PlatformFunction,
};

use super::{
    call_stack::StackFrame,
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
        Ok(Interpreter {
            fragments: Fragments::new(),
            evaluator: Evaluator::new(Module::default()),
            state: RuntimeState::Finished,
        })
    }

    pub fn evaluator(&mut self) -> &mut Evaluator<C> {
        &mut self.evaluator
    }

    pub fn register_platform(
        &mut self,
        functions: impl IntoIterator<Item = (PlatformFunction<C>, &'static str)>,
    ) {
        self.evaluator.global_namespace.register_platform(functions);
    }

    pub fn update(
        &mut self,
        code: &str,
        parent: Option<FragmentId>,
        scripts: &Scripts,
    ) -> Result<FragmentId, PipelineError> {
        // I'm currently working on adding compile-time evaluation to the
        // pipeline, meaning that at the end of the pipeline, the top-level
        // context is evaluated, so any functions and modules defined in the
        // code are known then. This aids in the update process, which is
        // currently buggy around some edge cases.
        //
        // (Currently, the top-level context is evaluated at runtime. To make
        // room for a compile-time top-level context, we need a `main` function
        // as an entry point for the runtime. But I don't believe that's
        // relevant to what's happening here. I'm just mentioning it for the
        // sake of completeness.)
        //
        // There are many changes required to make this work, many of which I've
        // already made. One that's not completed yet, is preloading all scripts
        // that could be loaded as modules by the entry script. This is required
        // to make compile-time module loading work, as the pipeline can't rely
        // on a filesystem being present.
        //
        // (It would also be possible to have the pipeline signal the platform
        // that it should load the module using whatever platform-specific
        // means are appropriate. But pre-loading seems simpler, and I can't see
        // a reason why it wouldn't be good enough.)
        //
        // So, the idea is that the platform pre-loads all scripts within the
        // current directory tree, then passes all of that to the pipeline, and
        // `mod` is a special pipeline platform function that then runs another
        // recursive pipeline on the declared module.
        //
        // So far, so good. The problem lies with how this method is called,
        // currently, which is for every watched file that changes. Right now,
        // this works because the pipeline doesn't really produce anything
        // except new fragments, and the (currently buggy) update process is
        // performed based on those.
        //
        // But the future (planned to be no longer buggy) update process is
        // meant to be performed based on the `Module` returned by the pipeline.
        // The specific improvement that comes from that, is that it becomes
        // easy to compare the old `Module` against the new one and see which
        // functions (and modules) have been removed.
        //
        // But here comes the problem: If this method is initially called with
        // the code from the entry script, we're getting a `Module` with
        // everything that's there. If it's later called again with the
        // (changed) code of some other script deeper in the tree, the resulting
        // `Module` will only have whatever that script defined. Which will be
        // much less, potentially, meaning we no longer have a reliable means of
        // determining what was removed, which was the whole point of the
        // exercise.
        //
        // So, what can we do? I see two possible solutions:
        //
        // 1. Move away from the one global namespace, which we need to do
        //    anyway at some point. Then we can compare the old `Module` for
        //    this specific script with the new one, and get a complete picture.
        //
        //    What I'm worried about though, is that this will require work that
        //    will actually have to be re-done in large parts, when I'm making
        //    the pipeline changes afterwards. Maybe some of it will make the
        //    pipeline changes harder.
        //
        // 2. Re-compile the entry script (and thus *everything*) on every
        //    change to any script. This sounds doable, and definitely good
        //    enough for now, but it will require changes to how this function
        //    is called. Maybe to the whole API of this struct.
        //
        // Maybe it's a good idea to implement the pre-loading change first, see
        // if it's possible to prepare the API here for what's to come, then
        // implement the pipeline changes afterwards.

        let PipelineOutput { start, module } =
            pipeline::run(code, parent, &mut self.fragments, scripts)?;
        dbg!(&module);

        for Replacement { old, new } in self.fragments.take_replacements() {
            self.evaluator.call_stack.replace(old, new);
            self.evaluator.data_stack.replace(old, new);
            self.evaluator
                .global_namespace
                .replace(old, new, &self.fragments);
        }

        if self.state.finished() {
            // Restart the program.
            self.evaluator
                .call_stack
                .push(StackFrame::Fragment { fragment_id: start });
        }

        Ok(start)
    }

    pub fn step(
        &mut self,
        platform_context: &mut C,
    ) -> Result<RuntimeState, EvaluatorError> {
        self.state = self.evaluator.step(&self.fragments, platform_context)?;
        Ok(self.state)
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
            self.evaluator.call_stack.push(StackFrame::Fragment {
                fragment_id: function.body.start,
            });
            self.evaluator.data_stack.clear();

            while !self
                .evaluator
                .step(&self.fragments, platform_context)?
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
        pipeline::{PipelineError, Scripts},
        runtime::evaluator::EvaluatorError,
        value, DataStackResult, PlatformFunction, PlatformFunctionState,
        RuntimeContext,
    };

    // Make sure all updates happen in the middle of their respective context,
    // not the beginning. This is the more complex case, and leads to the test
    // exercising more of the relevant machinery.

    #[test]
    fn update_to_named_function() -> anyhow::Result<()> {
        let mut interpreter = Interpreter::new()?;

        let original = "
            :f { nop 1 ping f } fn
            :main { f } fn
            main";
        let updated = "
            :f { nop 2 ping f } fn
            :main { f } fn
            main";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_to_identical_functions() -> anyhow::Result<()> {
        let mut interpreter = Interpreter::new()?;

        let original = "
            :loop { f loop } fn
            :f { nop 1 ping } fn
            :g { nop 1 ping } fn
            :main { loop } fn
            main";
        let updated = "
            :loop { g loop } fn
            :f { nop 2 ping } fn
            :g { nop 1 ping } fn
            :main { loop } fn
            main";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }

    #[test]
    fn update_that_reverts_back_to_an_earlier_version() -> anyhow::Result<()> {
        let mut interpreter = Interpreter::new()?;

        let original = "
            :f { nop 1 ping f } fn
            :main { f } fn
            main";
        let updated = "
            :f { nop 2 ping f } fn
            :main { f } fn
            main";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }

    #[test]
    fn update_to_block() -> anyhow::Result<()> {
        let mut interpreter = Interpreter::new()?;

        let original = "
            :main
            {
                { nop 1 ping }
                    clone
                    eval
                    eval
            }
                fn

            main";
        let updated = "
            :main
            {
                { nop 2 ping }
                    clone
                    eval
                    eval
            }
                fn

            main";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_to_identical_blocks_at_end_of_context() -> anyhow::Result<()> {
        let mut interpreter = Interpreter::new()?;

        let original = "
            :f { { nop 2 ping } } fn
            :g { { nop 2 ping } } fn

            :main
            {
                1 ping
                f eval
                g eval
            }
                fn

            main";
        let updated = "
            :f { { nop 2 ping } } fn
            :g { { nop 3 ping } } fn

            :main
            {
                1 ping
                f eval
                g eval
            }
                fn

            main";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;
        interpreter.wait_for_ping_on_channel(3)?;

        Ok(())
    }

    #[test]
    fn update_function_caller() -> anyhow::Result<()> {
        let mut interpreter = Interpreter::new()?;

        let original = "
            :f { nop 1 ping } fn

            :main
            {
                f
                1 ping
            }
                fn

            main";
        let updated = "
            :f { nop 1 ping } fn

            :main
            {
                f
                2 ping
            }
                fn

            main";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_renamed_function() -> anyhow::Result<()> {
        let mut interpreter = Interpreter::new()?;

        let original = "
            :f { nop 1 ping f } fn
            :main { f } fn
            main";
        let updated = "
            :g { nop 1 ping g } fn
            :main { g } fn
            main";

        interpreter.update(original)?;
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
        pub fn new() -> anyhow::Result<Self> {
            let mut inner = crate::Interpreter::new()?;
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
            let parent = None;
            let scripts = Scripts::default();

            self.inner.update(code, parent, &scripts)?;

            Ok(())
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
    ) -> DataStackResult<PlatformFunctionState> {
        let (channel, _) =
            runtime_context.data_stack.pop_specific::<value::Number>()?;
        *platform_context.channels.entry(channel.0).or_insert(0) += 1;
        Ok(PlatformFunctionState::Done)
    }
}
