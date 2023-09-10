use crate::language::{
    intrinsics::Context,
    repr::eval::fragments::{FragmentPayload, Fragments},
};

use super::{
    call_stack::CallStack,
    data_stack::{DataStack, DataStackError},
    functions::{self, Function, Functions, ResolveError},
};

#[derive(Debug)]
pub struct Evaluator {
    pub context: Context,
    pub functions: Functions,
    pub call_stack: CallStack,
    pub data_stack: DataStack,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
            functions: Functions::new(),
            call_stack: CallStack::new(),
            data_stack: DataStack::new(),
        }
    }

    pub fn step(
        &mut self,
        fragments: &Fragments,
    ) -> Result<EvaluatorState, EvaluatorError> {
        let syntax_fragment = match self.call_stack.current() {
            Some(fragment_id) => fragments.get(fragment_id),
            None => return Ok(EvaluatorState::Finished),
        };

        // We're done with the call stack for this step. Let's advance it now,
        // so that's out of the way.
        self.call_stack.advance(syntax_fragment.next());

        match &syntax_fragment.payload {
            FragmentPayload::Word(word) => {
                match self.functions.resolve(word)? {
                    Function::Intrinsic(intrinsic) => intrinsic(self)?,
                    Function::UserDefined(functions::UserDefined { body }) => {
                        if let Some(start) = body.start {
                            self.call_stack.push(start);
                            return Ok(EvaluatorState::InProgress);
                        }
                    }
                }
            }
            FragmentPayload::Value(value) => {
                self.data_stack.push(value.clone());
            }
        };

        Ok(EvaluatorState::InProgress)
    }
}

pub enum EvaluatorState {
    InProgress,
    Finished,
}

impl EvaluatorState {
    pub fn in_progress(&self) -> bool {
        matches!(self, Self::InProgress)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error("Error operating data stack")]
    DataStack(#[from] DataStackError),

    #[error("Error resolving function")]
    ResolveFunction(#[from] ResolveError),
}
