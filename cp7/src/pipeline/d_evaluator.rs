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
    pub fn new(start: SyntaxHandle) -> Self {
        Self {
            functions: Functions::new(),
            call_stack: CallStack::new(start),
            data_stack: DataStack::new(),
        }
    }
}

pub fn evaluate(start: SyntaxHandle, syntax: Syntax) -> anyhow::Result<()> {
    let mut evaluator = Evaluator::new(start);

    evaluate_syntax(&mut evaluator, &syntax)?;

    Ok(())
}

pub fn evaluate_syntax(
    evaluator: &mut Evaluator,
    syntax: &Syntax,
) -> anyhow::Result<()> {
    while let Some(handle) = evaluator.call_stack.current() {
        let fragment = syntax.get(handle);

        match fragment.payload {
            SyntaxElement::FnRef(fn_ref) => {
                match evaluator.functions.resolve(&fn_ref)? {
                    Function::Intrinsic(intrinsic) => intrinsic(
                        &mut evaluator.functions,
                        &mut evaluator.data_stack,
                    )?,
                    Function::UserDefined { body } => {
                        if let Some(body) = body.0 {
                            evaluator.call_stack.push(body);
                            continue;
                        }
                    }
                }
            }
            SyntaxElement::Value(value) => {
                evaluator.data_stack.push(value);
            }
        };

        match fragment.next {
            Some(handle) => {
                evaluator.call_stack.update(handle);
            }
            None => {
                evaluator.call_stack.pop();
            }
        }
    }

    Ok(())
}
