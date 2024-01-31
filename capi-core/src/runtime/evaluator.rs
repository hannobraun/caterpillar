use crate::{
    pipeline::{Function, Module},
    platform::Platform,
    repr::eval::{
        fragments::{FragmentId, FragmentPayload, Fragments},
        value::Value,
    },
    PlatformFunctionState,
};

use super::{
    call_stack::{CallStack, StackFrame},
    data_stack::{DataStack, DataStackError},
    namespaces::{
        IntrinsicFunctionState, ItemInModule, Namespace, ResolveError,
        RuntimeContext,
    },
};

#[derive(Debug)]
pub struct Evaluator<P: Platform> {
    pub global_namespace: Namespace<P::Context>,
    pub call_stack: CallStack,
    pub data_stack: DataStack,
    pub side_stack: DataStack,
}

impl<P: Platform> Evaluator<P> {
    pub fn new(module: Module) -> Self {
        let mut global_namespace = Namespace::new(module);

        global_namespace.register_platform(P::functions());

        Self {
            global_namespace,
            call_stack: Default::default(),
            data_stack: Default::default(),
            side_stack: Default::default(),
        }
    }

    pub fn step(
        &mut self,
        fragments: &mut Fragments,
        platform_context: &mut P::Context,
    ) -> Result<RuntimeState, EvaluatorError> {
        // What we need to do next depends on what's on top of the call stack.
        // Let's get the current stack frame off, and replace it with a stack
        // frame that points to the subsequent piece of code.
        //
        // That way, whether we want to execute that subsequent piece of code
        // next, or whether we end up adding more stack frames first, we can be
        // sure that the current stack frame already points to the correct
        // place.
        let current_stack_frame = {
            let Some(current_stack_frame) = self.call_stack.pop() else {
                return Ok(RuntimeState::Finished);
            };

            match current_stack_frame {
                StackFrame::Main { .. } => {
                    if let Ok(ItemInModule::UserDefinedFunction(main)) =
                        self.global_namespace.resolve("main")
                    {
                        self.call_stack.push(StackFrame::Fragment {
                            fragment_id: main.body.start,
                        })
                    }

                    // If there's no `main` function, there's nothing to run.
                    //
                    // This could lead to silent errors, if someone misspelled
                    // `main` or something like that, but it's okay for now.
                    // `StackFrame::Main` is just a stopgap anyway.
                }
                StackFrame::Fragment { fragment_id } => {
                    let fragment = fragments.get(fragment_id);

                    if let Some(next) = fragment.next() {
                        self.call_stack
                            .push(StackFrame::Fragment { fragment_id: next });
                    };
                }
                StackFrame::IntrinsicFunction {
                    word,
                    function,
                    step,
                } => {
                    self.call_stack.push(StackFrame::IntrinsicFunction {
                        word,
                        function,
                        step: step + 1,
                    });
                }
            }

            current_stack_frame
        };

        let runtime_state = match current_stack_frame {
            StackFrame::Main { .. } => {
                // Nothing to do. Already set the appropriate next stack frame
                // above.
                RuntimeState::Running
            }
            StackFrame::Fragment { fragment_id } => {
                let fragment = fragments.get(fragment_id);

                match &fragment.payload {
                    FragmentPayload::Value(value) => {
                        self.data_stack.push(Value {
                            payload: value.clone(),
                            fragment: Some(fragment_id),
                        });
                        RuntimeState::Running
                    }
                    FragmentPayload::Word(word) => {
                        let item_in_namespace = self
                            .global_namespace
                            .resolve(word)
                            .map_err(|err| EvaluatorError {
                                kind: err.into(),
                                fragment: fragment_id,
                            })?;

                        match item_in_namespace {
                            ItemInModule::Binding(value) => {
                                self.data_stack.push(value);
                                RuntimeState::Running
                            }
                            ItemInModule::IntrinsicFunction(f) => {
                                self.call_stack.push(
                                    StackFrame::IntrinsicFunction {
                                        word: fragment_id,
                                        function: f,
                                        step: 0,
                                    },
                                );

                                RuntimeState::Running
                            }
                            ItemInModule::PlatformFunction(f) => {
                                let function_state = f(
                                    self.runtime_context(
                                        fragment_id,
                                        fragments,
                                    ),
                                    platform_context,
                                )
                                .map_err(|err| EvaluatorError {
                                    kind: err.into(),
                                    fragment: fragment_id,
                                })?;

                                match function_state {
                                    PlatformFunctionState::Done => {
                                        RuntimeState::Running
                                    }
                                    PlatformFunctionState::Sleeping => {
                                        RuntimeState::Sleeping
                                    }
                                }
                            }
                            ItemInModule::UserDefinedFunction(Function {
                                body,
                                ..
                            }) => {
                                self.call_stack.push(StackFrame::Fragment {
                                    fragment_id: body.start,
                                });
                                RuntimeState::Running
                            }
                        }
                    }
                    FragmentPayload::Terminator => {
                        // Nothing to do here. Terminators only exist for
                        // fragment addressing purposes and don't need to be
                        // handled during evaluation.
                        RuntimeState::Running
                    }
                }
            }
            StackFrame::IntrinsicFunction {
                word,
                function,
                step,
            } => {
                let state =
                    function(step, self.runtime_context(word, fragments))
                        .map_err(|err| EvaluatorError {
                            kind: err.into(),
                            fragment: word,
                        })?;

                match state {
                    IntrinsicFunctionState::StepDone => {
                        // Nothing to do. We already advanced the stack frame.
                    }
                    IntrinsicFunctionState::FullyCompleted => {
                        self.call_stack.pop();
                    }
                }

                RuntimeState::Running
            }
        };

        Ok(runtime_state)
    }

    fn runtime_context<'r>(
        &'r mut self,
        word: FragmentId,
        fragments: &'r mut Fragments,
    ) -> RuntimeContext<'r> {
        RuntimeContext {
            word,
            fragments,
            global_module: self.global_namespace.global_module(),
            call_stack: &mut self.call_stack,
            data_stack: &mut self.data_stack,
            side_stack: &mut self.side_stack,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RuntimeState {
    Running,
    Sleeping,
    Finished,
}

impl RuntimeState {
    pub fn finished(&self) -> bool {
        matches!(self, Self::Finished)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Evaluator error at `{}`", .fragment.display_short())]
pub struct EvaluatorError {
    #[source]
    pub kind: EvaluatorErrorKind,

    pub fragment: FragmentId,
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorErrorKind {
    #[error("Error operating data stack")]
    DataStack(#[from] DataStackError),

    #[error(transparent)]
    ResolveFunction(#[from] ResolveError),
}
