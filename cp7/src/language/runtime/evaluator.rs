use crate::language::repr::eval::fragments::{FragmentPayload, Fragments};

use super::{
    call_stack::CallStack,
    data_stack::{DataStack, DataStackError},
    functions::{self, Function, Functions, ResolveError},
};

#[derive(Debug)]
pub struct Evaluator {
    pub functions: Functions,
    pub call_stack: CallStack,
    pub data_stack: DataStack,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
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

        let next_fragment = syntax_fragment.next();

        match syntax_fragment.payload {
            FragmentPayload::Word(word) => {
                match self.functions.resolve(&word)? {
                    Function::Intrinsic(intrinsic) => {
                        intrinsic(&mut self.functions, &mut self.data_stack)?
                    }
                    Function::UserDefined(functions::UserDefined { body }) => {
                        if let Some(start) = body.start {
                            // Need to advance the current stack frame before we
                            // jump into the function, or we'll repeat it
                            // endlessly when we come back.
                            self.call_stack.advance(next_fragment);
                            self.call_stack.push(start);
                            return Ok(EvaluatorState::InProgress);
                        }
                    }
                }
            }
            FragmentPayload::Value(value) => {
                self.data_stack.push(value);
            }
        };

        self.call_stack.advance(next_fragment);

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
