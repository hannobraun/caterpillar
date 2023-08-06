use crate::{
    functions::{Function, Functions, ResolveError},
    runtime::{
        call_stack::CallStack,
        data_stack::{DataStack, DataStackError},
    },
    syntax::{Syntax, SyntaxElement, SyntaxHandle},
};

pub struct Evaluator {
    functions: Functions,
    call_stack: CallStack,
    data_stack: DataStack,
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
        handle: SyntaxHandle,
        syntax: &Syntax,
    ) -> Result<(), EvaluatorError> {
        let fragment = syntax.get(handle);

        match fragment.payload {
            SyntaxElement::FnRef(fn_ref) => {
                match self.functions.resolve(&fn_ref)? {
                    Function::Intrinsic(intrinsic) => {
                        intrinsic(&mut self.functions, &mut self.data_stack)?
                    }
                    Function::UserDefined { body } => {
                        if let Some(body) = body.0 {
                            self.call_stack.push(body);
                            return Ok(());
                        }
                    }
                }
            }
            SyntaxElement::Value(value) => {
                self.data_stack.push(value);
            }
        };

        match fragment.next {
            Some(handle) => {
                self.call_stack.update(handle);
            }
            None => {
                self.call_stack.pop();
            }
        }

        Ok(())
    }
}

pub fn evaluate_syntax(
    evaluator: &mut Evaluator,
    syntax: &Syntax,
) -> anyhow::Result<()> {
    while let Some(handle) = evaluator.call_stack.current() {
        evaluator.step(handle, syntax)?;
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error("Error operating data stack")]
    DataStack(#[from] DataStackError),

    #[error("Error resolving function")]
    ResolveFunction(#[from] ResolveError),
}
