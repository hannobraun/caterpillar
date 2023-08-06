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

    evaluate_syntax(&syntax, &mut functions, &mut call_stack, &mut data_stack)?;

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

        evaluate_syntax_element(
            fragment.payload,
            syntax,
            functions,
            call_stack,
            data_stack,
        )?;

        call_stack.update(fragment.next);
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
                if let Some(body) = body.0 {
                    call_stack.update(Some(body));
                    evaluate_syntax(syntax, functions, call_stack, data_stack)?;
                }
            }
        },
        SyntaxElement::Value(value) => {
            data_stack.push(value);
        }
    }

    Ok(())
}
