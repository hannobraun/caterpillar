use crate::{
    functions::{Function, Functions},
    runtime::{call_stack::CallStack, data_stack::DataStack},
    syntax::{Syntax, SyntaxElement, SyntaxHandle},
};

pub struct Evaluator {
    functions: Functions,
    call_stack: CallStack,
    data_stack: DataStack,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            functions: Functions::new(),
            call_stack: CallStack::new(),
            data_stack: DataStack::new(),
        }
    }
}

pub fn evaluate(
    start: Option<SyntaxHandle>,
    syntax: Syntax,
) -> anyhow::Result<()> {
    let mut evaluator = Evaluator::new();

    if let Some(start) = start {
        evaluator.call_stack.push(start);
        evaluate_syntax(
            &syntax,
            &mut evaluator.functions,
            &mut evaluator.call_stack,
            &mut evaluator.data_stack,
        )?;
    }

    Ok(())
}

fn evaluate_syntax(
    syntax: &Syntax,
    functions: &mut Functions,
    call_stack: &mut CallStack,
    data_stack: &mut DataStack,
) -> anyhow::Result<()> {
    while let Some(handle) = call_stack.current() {
        let fragment = syntax.get(handle);

        match fragment.payload {
            SyntaxElement::FnRef(fn_ref) => {
                match functions.resolve(&fn_ref)? {
                    Function::Intrinsic(intrinsic) => {
                        intrinsic(functions, data_stack)?
                    }
                    Function::UserDefined { body } => {
                        if let Some(body) = body.0 {
                            call_stack.push(body);
                            evaluate_syntax(
                                syntax, functions, call_stack, data_stack,
                            )?;
                        }
                    }
                }
            }
            SyntaxElement::Value(value) => {
                data_stack.push(value);
            }
        };

        match fragment.next {
            Some(handle) => {
                call_stack.update(handle);
            }
            None => {
                call_stack.pop();
            }
        }
    }

    Ok(())
}
