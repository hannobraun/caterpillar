use crate::{
    repr::eval::{
        fragments::{FragmentId, FragmentPayload, Fragments},
        value::Value,
    },
    FunctionState,
};

use super::{
    call_stack::{CallStack, StackFrame},
    data_stack::{DataStack, DataStackError},
    namespaces::{
        ItemInModule, Namespace, ResolveError, RuntimeContext,
        UserDefinedFunction,
    },
};

#[derive(Debug)]
pub struct Evaluator<C> {
    pub global_namespace: Namespace<C>,
    pub call_stack: CallStack,
    pub data_stack: DataStack,
}

impl<C> Evaluator<C> {
    pub fn step(
        &mut self,
        fragments: &mut Fragments,
        platform_context: &mut C,
    ) -> Result<RuntimeState, EvaluatorError> {
        let Some(stack_frame) = self.call_stack.current() else {
            return Ok(RuntimeState::Finished);
        };

        let StackFrame::Fragment(fragment_id) = stack_frame;
        let fragment = fragments.get(fragment_id);

        // We're done with the call stack for this step. Let's advance it now,
        // so that's out of the way.
        self.call_stack.advance(fragment.next());

        let runtime_state = match &fragment.payload {
            FragmentPayload::Array { start } => {
                // Remember the current stack frame, so we know when we're done
                // evaluating the array.
                let current = self.call_stack.current();

                // Evaluate the array.
                self.data_stack.mark();
                self.call_stack.push(*start);
                while current != self.call_stack.current() {
                    self.step(fragments, platform_context)?;
                }

                let items = self
                    .data_stack
                    .drain_values_from_marker()
                    .map(|value| value.payload)
                    .collect();

                self.data_stack.push(Value {
                    payload: crate::value::ValuePayload::Array(items),
                    fragment: Some(fragment_id),
                });

                RuntimeState::Running
            }
            FragmentPayload::Value(value) => {
                self.data_stack.push(Value {
                    payload: value.clone(),
                    fragment: Some(fragment_id),
                });

                RuntimeState::Running
            }
            FragmentPayload::Word(word) => {
                let function_state = match self
                    .global_namespace
                    .resolve(word)
                    .map_err(|err| EvaluatorError {
                    kind: err.into(),
                    fragment: fragment_id,
                })? {
                    ItemInModule::Binding(value) => {
                        self.data_stack.push(value);
                        FunctionState::Done
                    }
                    ItemInModule::IntrinsicFunction(f) => {
                        let step = 0;
                        f(step, self.runtime_context(fragment_id, fragments))
                            .map_err(|err| EvaluatorError {
                            kind: err.into(),
                            fragment: fragment_id,
                        })?;
                        FunctionState::Done
                    }
                    ItemInModule::PlatformFunction(f) => f(
                        self.runtime_context(fragment_id, fragments),
                        platform_context,
                    )
                    .map_err(|err| EvaluatorError {
                        kind: err.into(),
                        fragment: fragment_id,
                    })?,
                    ItemInModule::UserDefinedFunction(
                        UserDefinedFunction { body, .. },
                    ) => {
                        self.call_stack.push(body.start);
                        FunctionState::Done
                    }
                };

                match function_state {
                    FunctionState::Done => RuntimeState::Running,
                    FunctionState::Sleeping => RuntimeState::Sleeping,
                }
            }
            FragmentPayload::Terminator => {
                // Nothing to do here. Terminators only exist for fragment
                // addressing purposes and don't need to be handled during
                // evaluation.
                RuntimeState::Running
            }
        };

        Ok(runtime_state)
    }

    fn runtime_context<'r>(
        &'r mut self,
        this: FragmentId,
        fragments: &'r mut Fragments,
    ) -> RuntimeContext<'r> {
        RuntimeContext {
            this,
            fragments,
            namespace: self.global_namespace.user_defined(),
            call_stack: &mut self.call_stack,
            data_stack: &mut self.data_stack,
        }
    }
}

impl<C> Default for Evaluator<C> {
    fn default() -> Self {
        Self {
            global_namespace: Default::default(),
            call_stack: Default::default(),
            data_stack: Default::default(),
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
