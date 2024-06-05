use std::fmt;

use crate::{
    pipeline::{Function, Module},
    platform::{BuiltinFnError, BuiltinFnState, CoreContext, Platform},
    repr::eval::{
        fragments::{FragmentId, FragmentPayload, Fragments},
        value::Value,
    },
};

use super::{
    call_stack::{CallStack, StackFrame},
    data_stack::{DataStack, DataStackError},
    namespaces::{ItemInModule, Namespace, ResolveError},
};

pub struct Evaluator<P: Platform> {
    pub global_namespace: Namespace<P>,
    pub call_stack: CallStack,
    pub data_stack: DataStack,
    pub side_stack: DataStack,
}

impl<P: Platform> Evaluator<P> {
    pub fn new(module: Module) -> Self {
        let mut global_namespace = Namespace::new(module);

        global_namespace.register_platform(P::builtin_fns());

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
        platform_context: &mut P::Context<'_>,
    ) -> Result<RuntimeState, EvaluatorError<P::Error>> {
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
                                call_stack: self.call_stack.clone(),
                                fragments: fragments.clone(),
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
                                    0, // placeholder!
                                    self.runtime_context(
                                        fragment_id,
                                        fragments,
                                    ),
                                    platform_context,
                                )
                                .map_err(|err| EvaluatorError {
                                    kind: err.into(),
                                    fragment: fragment_id,
                                    call_stack: self.call_stack.clone(),
                                    fragments: fragments.clone(),
                                })?;

                                match function_state {
                                    BuiltinFnState::Completed => {
                                        RuntimeState::Running
                                    }
                                    BuiltinFnState::Sleeping => {
                                        RuntimeState::Sleeping
                                    }
                                    BuiltinFnState::Stepped => {
                                        unreachable!()
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
                let state = function(
                    step,
                    self.runtime_context(word, fragments),
                    &mut (),
                )
                .map_err(|err| EvaluatorError {
                    kind: EvaluatorErrorKind::CoreBuiltinFn(err),
                    fragment: word,
                    call_stack: self.call_stack.clone(),
                    fragments: fragments.clone(),
                })?;

                match state {
                    BuiltinFnState::Completed => {
                        self.call_stack.pop();
                    }
                    BuiltinFnState::Sleeping => {
                        unreachable!()
                    }
                    BuiltinFnState::Stepped => {
                        // Nothing to do. We already advanced the stack frame.
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
    ) -> CoreContext<'r> {
        CoreContext {
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
pub struct EvaluatorError<E> {
    #[source]
    pub kind: EvaluatorErrorKind<E>,

    pub fragment: FragmentId,
    pub call_stack: CallStack,
    pub fragments: Fragments,
}

impl<E> EvaluatorError<E> {
    pub fn assert_resolve(self) -> ResolveError {
        if let EvaluatorErrorKind::Resolve(err) = self.kind {
            return err;
        }

        panic!("Expected `ResolveError`")
    }
}

impl<E> fmt::Display for EvaluatorError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Evaluator error at `{}`", self.fragment.display_short())?;

        write!(f, "\nCall stack:")?;
        for (i, stack_frame) in self.call_stack.iter().enumerate() {
            writeln!(f)?;

            write!(f, "  {:2}. ", i + 1)?;

            match stack_frame {
                StackFrame::Fragment { fragment_id } => {
                    let fragment = self.fragments.get(*fragment_id);
                    write!(
                        f,
                        "{:?} (`{}`)",
                        fragment.payload,
                        fragment_id.display_short(),
                    )?;
                }
                StackFrame::IntrinsicFunction { word, step, .. } => {
                    let fragment = self.fragments.get(*word);
                    write!(
                        f,
                        "intrinsic {:?} (`{}`) at step {step}",
                        fragment.payload,
                        word.display_short(),
                    )?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorErrorKind<E> {
    #[error("Error originating from core builtin function")]
    CoreBuiltinFn(#[source] BuiltinFnError<()>),

    #[error("Error originating from builtin function")]
    BuiltinFn(#[from] BuiltinFnError<E>),

    #[error("Error operating data stack")]
    DataStack(#[from] DataStackError),

    #[error(transparent)]
    Resolve(#[from] ResolveError),
}
