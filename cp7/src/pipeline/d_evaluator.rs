use crate::{
    functions::{Function, Functions},
    runtime::{call_stack::CallStack, data_stack::DataStack},
    syntax::{Syntax, SyntaxElement, SyntaxHandle},
};

pub fn evaluate(
    start: Option<SyntaxHandle>,
    syntax: Syntax,
) -> anyhow::Result<()> {
    let mut functions = Functions::new();
    let mut call_stack = CallStack::new(start);
    let mut data_stack = DataStack::new();

    evaluate_syntax(
        start,
        &syntax,
        &mut functions,
        &mut call_stack,
        &mut data_stack,
    )?;

    Ok(())
}

fn evaluate_syntax(
    start: Option<SyntaxHandle>,
    syntax: &Syntax,
    functions: &mut Functions,
    call_stack: &mut CallStack,
    data_stack: &mut DataStack,
) -> anyhow::Result<()> {
    call_stack.current = start;

    while let Some(handle) = call_stack.current() {
        let fragment = syntax.get(handle);

        evaluate_syntax_element(
            fragment.payload,
            syntax,
            functions,
            call_stack,
            data_stack,
        )?;

        call_stack.current = fragment.next;
    }

    Ok(())
}

fn evaluate_syntax_element(
    syntax_element: SyntaxElement,
    syntax: &Syntax,
    functions: &mut Functions,
    call_stack: &mut CallStack,
    data_stack: &mut DataStack,
) -> anyhow::Result<()> {
    match syntax_element {
        SyntaxElement::FnRef(fn_ref) => match functions.resolve(&fn_ref)? {
            Function::Intrinsic(intrinsic) => intrinsic(functions, data_stack)?,
            Function::UserDefined { body } => {
                evaluate_syntax(
                    body.0, syntax, functions, call_stack, data_stack,
                )?;
            }
        },
        SyntaxElement::Value(value) => {
            data_stack.push(value);
        }
    }

    Ok(())
}
