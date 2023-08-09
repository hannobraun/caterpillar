use crate::language::{
    functions::{Function, Functions, ResolveError},
    syntax::{Syntax, SyntaxElement, SyntaxHandle},
};

use super::{
    call_stack::CallStack,
    data_stack::{DataStack, DataStackError},
};

#[derive(Debug)]
pub struct Evaluator {
    pub functions: Functions,
    pub call_stack: CallStack,
    pub data_stack: DataStack,
}

impl Evaluator {
    pub fn new(start: SyntaxHandle) -> Self {
        Self {
            functions: Functions::new(),
            call_stack: CallStack::new(start),
            data_stack: DataStack::new(),
        }
    }

    pub fn step(
        &mut self,
        syntax: &Syntax,
    ) -> Result<EvaluatorState, EvaluatorError> {
        let syntax_fragment = match self.call_stack.current() {
            Some(handle) => syntax.get(handle),
            None => return Ok(EvaluatorState::Finished),
        };

        match syntax_fragment.payload {
            SyntaxElement::Word(word) => {
                match self.functions.resolve(&word)? {
                    Function::Intrinsic(intrinsic) => {
                        intrinsic(&mut self.functions, &mut self.data_stack)?
                    }
                    Function::UserDefined { body } => {
                        if let Some(body) = body.0 {
                            self.call_stack.push(body);
                            return Ok(EvaluatorState::InProgress);
                        }
                    }
                }
            }
            SyntaxElement::Value(value) => {
                self.data_stack.push(value);
            }
        };

        match syntax_fragment.next {
            Some(handle) => {
                self.call_stack.update(handle);
            }
            None => {
                self.call_stack.pop();
            }
        }

        Ok(EvaluatorState::InProgress)
    }
}

pub enum EvaluatorState {
    InProgress,
    Finished,
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error("Error operating data stack")]
    DataStack(#[from] DataStackError),

    #[error("Error resolving function")]
    ResolveFunction(#[from] ResolveError),
}
