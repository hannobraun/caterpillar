use crate::{
    pipeline::{self, Module, PipelineError, PipelineOutput, Scripts},
    platform::Platform,
    repr::eval::{
        fragments::{FragmentId, Fragments, Replacement},
        value,
    },
    runtime::namespaces::ItemInModule,
};

use super::{
    call_stack::StackFrame,
    data_stack::{DataStack, DataStackError},
    evaluator::{Evaluator, EvaluatorError, RuntimeState},
};

// This API is in the middle of a refactor. Here's what remains to be done:
//
// - Change `update` to take `ScriptPath` and `String`, increasing its
//   granularity.
// - Then, in `update`, update the `Scripts` instance and re-run the pipeline.
// - Make `scripts` field private.`

pub struct Interpreter<P: Platform> {
    fragments: Fragments,
    evaluator: Evaluator<P>,
    state: RuntimeState,

    pub scripts: Scripts,
}

impl<P: Platform> Interpreter<P> {
    pub fn new(scripts: Scripts) -> Result<Self, PipelineError> {
        Ok(Interpreter {
            fragments: Fragments::new(),
            evaluator: Evaluator::new(Module::default()),
            state: RuntimeState::Finished,
            scripts,
        })
    }

    pub fn evaluator(&mut self) -> &mut Evaluator<P> {
        &mut self.evaluator
    }

    pub fn update(&mut self) -> Result<FragmentId, PipelineError> {
        let code = self
            .scripts
            .inner
            .get(&self.scripts.entry_script_path)
            .expect("Code for entry script not found");
        let parent = None;

        let PipelineOutput { start, module } =
            pipeline::run(code, parent, &mut self.fragments, &self.scripts)?;

        *self.evaluator.global_namespace.global_module() = module;

        for Replacement { old, new } in self.fragments.take_replacements() {
            self.evaluator.call_stack.replace(old, new);
            self.evaluator.data_stack.replace(old, new);
        }

        if self.state.finished() {
            // Restart the program (i.e. run `main`, if available).
            //
            // We might not always have a `main` function. Either by design, if
            // this is a library module, and we're just running its tests, or by
            // accident, if the user misspelled it.
            //
            // It would be nice to be more explicit about what we're trying to
            // do with the module, so we can detect misspellings. But for now,
            // this will do.
            if let Ok(ItemInModule::UserDefinedFunction(main)) =
                self.evaluator.global_namespace.resolve("main")
            {
                self.evaluator.call_stack.push(StackFrame::Fragment {
                    fragment_id: main.body.start,
                })
            }
        }

        Ok(start)
    }

    pub fn step(
        &mut self,
        platform_context: &mut P::Context<'_>,
    ) -> Result<RuntimeState, EvaluatorError<P::Error>> {
        self.state =
            self.evaluator.step(&mut self.fragments, platform_context)?;
        Ok(self.state)
    }

    pub fn run_tests(
        &mut self,
        mut platform_context: P::Context<'_>,
    ) -> Result<(), TestError<P::Error>> {
        while !self.step(&mut platform_context)?.finished() {}

        let tests = self
            .evaluator
            .global_namespace
            .global_module()
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
            //
            // (We have to clear the data stack before the next test run
            // though.)
            self.evaluator.call_stack.push(StackFrame::Fragment {
                fragment_id: function.body.start,
            });
            self.evaluator.data_stack.clear();

            while !self
                .evaluator
                .step(&mut self.fragments, &mut platform_context)?
                .finished()
            {}

            let (result, _) =
                self.evaluator.data_stack.pop_specific::<value::Bool>()?;

            if !self.evaluator.data_stack.is_empty() {
                return Err(TestError::DataStackNotEmptyAfterTestRun {
                    data_stack: self.evaluator.data_stack.clone(),
                });
            }

            if result.0 {
                println!(" PASS");
            } else {
                println!(" FAIL");
            }
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TestError<T> {
    #[error(transparent)]
    Evaluator(#[from] EvaluatorError<T>),

    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error(
        "Data stack not empty after evaluating test definitions: {data_stack}"
    )]
    DataStackNotEmptyAfterScriptEvaluation { data_stack: DataStack },

    #[error("Expected test to return one `bool`; left on stack: {data_stack}")]
    DataStackNotEmptyAfterTestRun { data_stack: DataStack },
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use crate::{
        pipeline::{PipelineError, ScriptPath, Scripts},
        platform::{
            BuiltinFn, BuiltinFnResult, BuiltinFnState, BuiltinFns,
            CoreContext, Platform,
        },
        repr::eval::value,
        runtime::evaluator::EvaluatorError,
    };

    // Make sure all updates happen in the middle of their respective context,
    // not the beginning. This is the more complex case, and leads to the test
    // exercising more of the relevant machinery.

    #[test]
    fn update_to_named_function() -> anyhow::Result<()> {
        let mut interpreter = TestInterpreter::new()?;

        let original = "
            :f { nop 1 ping f } fn
            :main { f } fn";
        let updated = "
            :f { nop 2 ping f } fn
            :main { f } fn";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_to_identical_functions() -> anyhow::Result<()> {
        let mut interpreter = TestInterpreter::new()?;

        let original = "
            :loop { f loop } fn
            :f { nop 1 ping } fn
            :g { nop 1 ping } fn
            :main { loop } fn";
        let updated = "
            :loop { g loop } fn
            :f { nop 2 ping } fn
            :g { nop 1 ping } fn
            :main { loop } fn";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }

    #[test]
    fn update_that_reverts_back_to_an_earlier_version() -> anyhow::Result<()> {
        let mut interpreter = TestInterpreter::new()?;

        let original = "
            :f { nop 1 ping f } fn
            :main { f } fn";
        let updated = "
            :f { nop 2 ping f } fn
            :main { f } fn";

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
        let mut interpreter = TestInterpreter::new()?;

        let original = "
            :main
            {
                { nop 1 ping }
                    clone
                    eval
                    eval
            }
                fn";
        let updated = "
            :main
            {
                { nop 2 ping }
                    clone
                    eval
                    eval
            }
                fn";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_to_identical_blocks_at_end_of_context() -> anyhow::Result<()> {
        let mut interpreter = TestInterpreter::new()?;

        let original = "
            :f { { nop 2 ping } } fn
            :g { { nop 2 ping } } fn

            :main
            {
                1 ping
                f eval
                g eval
            }
                fn";
        let updated = "
            :f { { nop 2 ping } } fn
            :g { { nop 3 ping } } fn

            :main
            {
                1 ping
                f eval
                g eval
            }
                fn";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;
        interpreter.wait_for_ping_on_channel(3)?;

        Ok(())
    }

    #[test]
    fn update_function_caller() -> anyhow::Result<()> {
        let mut interpreter = TestInterpreter::new()?;

        let original = "
            :f { nop 1 ping } fn

            :main
            {
                f
                1 ping
            }
                fn";
        let updated = "
            :f { nop 1 ping } fn

            :main
            {
                f
                2 ping
            }
                fn";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(2)?;

        Ok(())
    }

    #[test]
    fn update_renamed_function() -> anyhow::Result<()> {
        let mut interpreter = TestInterpreter::new()?;

        let original = "
            :f { nop 1 ping f } fn
            :main { f } fn";
        let updated = "
            :g { nop 1 ping g } fn
            :main { g } fn";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        interpreter.wait_for_ping_on_channel(1)?;

        Ok(())
    }

    // The functionality tested here doesn't work yet. Un-ignore it, once this
    // works.
    #[test]
    fn remove_function() -> anyhow::Result<()> {
        let mut interpreter = TestInterpreter::new()?;

        let original = "
            :f { nop 1 ping f } fn
            :main { f } fn";
        let updated = "
            :main { f } fn";

        interpreter.update(original)?;
        interpreter.wait_for_ping_on_channel(1)?;

        interpreter.update(updated)?;
        let err = interpreter.wait_for_error();

        let err = err.assert_resolve();
        assert_eq!(err.name, "f");

        Ok(())
    }

    struct TestInterpreter {
        entry_script_path: ScriptPath,
        inner: super::Interpreter<TestPlatform>,
        platform_context: PlatformContext,
    }

    impl TestInterpreter {
        pub fn new() -> anyhow::Result<Self> {
            let entry_script_path = vec![value::Symbol(String::from("entry"))];
            let scripts = Scripts {
                entry_script_path: entry_script_path.clone(),
                inner: BTreeMap::new(),
            };

            let inner = super::Interpreter::new(scripts)?;

            Ok(Self {
                entry_script_path,
                inner,
                platform_context: PlatformContext::default(),
            })
        }

        pub fn update(&mut self, code: &str) -> Result<(), PipelineError> {
            self.inner
                .scripts
                .inner
                .insert(self.entry_script_path.clone(), code.to_string());

            self.inner.update()?;

            Ok(())
        }

        pub fn wait_for_ping_on_channel(
            &mut self,
            channel: i64,
        ) -> Result<(), EvaluatorError<()>> {
            self.platform_context.channels.clear();

            let mut num_steps = 0;

            loop {
                if let Some(1) = self.platform_context.channels.get(&channel) {
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

        pub fn wait_for_error(&mut self) -> EvaluatorError<()> {
            let mut num_steps = 0;

            loop {
                if let Err(err) = self.inner.step(&mut self.platform_context) {
                    return err;
                }

                num_steps += 1;

                if num_steps == 1024 {
                    panic!("Waiting for error took too long");
                }
            }
        }
    }

    pub struct TestPlatform;

    impl Platform for TestPlatform {
        type Context<'r> = PlatformContext;
        type Error = ();

        fn builtin_fns() -> impl BuiltinFns<Self> {
            [(ping as BuiltinFn<Self>, "ping")]
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct PlatformContext {
        pub channels: HashMap<i64, i64>,
    }

    pub fn ping(
        step: usize,
        runtime_context: CoreContext,
        platform_context: &mut PlatformContext,
    ) -> BuiltinFnResult<()> {
        match step {
            0 => {
                let (channel, _) = runtime_context
                    .data_stack
                    .pop_specific::<value::Number>()?;
                *platform_context.channels.entry(channel.0).or_insert(0) += 1;
                Ok(BuiltinFnState::Completed)
            }
            _ => unreachable!(),
        }
    }
}
