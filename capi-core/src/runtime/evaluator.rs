use std::collections::HashMap;

use crate::repr::eval::{
    fragments::{FragmentPayload, Fragments},
    value::Value,
};

use super::{
    call_stack::CallStack,
    data_stack::{DataStack, DataStackError},
    functions::{self, Function, Functions, ResolveError},
};

#[derive(Debug, Default)]
pub struct Evaluator {
    pub functions: Functions,
    pub call_stack: CallStack,
    pub data_stack: DataStack,
}

impl Evaluator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> EvaluatorState {
        if self.call_stack.current().is_some() {
            EvaluatorState::InProgress
        } else {
            EvaluatorState::Finished
        }
    }

    pub fn step(
        &mut self,
        fragments: &Fragments,
        context: &mut Context,
    ) -> Result<EvaluatorState, EvaluatorError> {
        let (fragment_id, fragment) = match self.call_stack.current() {
            Some(fragment_id) => (fragment_id, fragments.get(fragment_id)),
            None => return Ok(EvaluatorState::Finished),
        };

        // We're done with the call stack for this step. Let's advance it now,
        // so that's out of the way.
        self.call_stack.advance(fragment.next());

        match &fragment.payload {
            FragmentPayload::Word(word) => {
                match self.functions.resolve(word)? {
                    Function::Native(f) => f(self, context)?,
                    Function::UserDefined(functions::UserDefined {
                        body,
                        ..
                    }) => {
                        self.call_stack.push(body.start);
                    }
                }
            }
            FragmentPayload::Value(value) => {
                self.data_stack.push(Value {
                    kind: value.clone(),
                    fragment: Some(fragment_id),
                });
            }
            FragmentPayload::Terminator => {
                // Nothing to do here. Terminators only exist for fragment
                // addressing purposes and don't need to be handled during
                // evaluation.
            }
        };

        Ok(EvaluatorState::InProgress)
    }
}

#[derive(Debug, Default)]
pub struct Context {
    pub channels: HashMap<i64, i64>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }
}

pub enum EvaluatorState {
    InProgress,
    Finished,
}

impl EvaluatorState {
    pub fn finished(&self) -> bool {
        matches!(self, Self::Finished)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error("Error operating data stack")]
    DataStack(#[from] DataStackError),

    #[error("Error resolving function")]
    ResolveFunction(#[from] ResolveError),
}
