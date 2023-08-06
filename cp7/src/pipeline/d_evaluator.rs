use crate::{
    data_stack::DataStack,
    functions::{Function, Functions},
    syntax::{Syntax, SyntaxElement, SyntaxHandle},
};

pub fn evaluate(
    start: Option<SyntaxHandle>,
    syntax: Syntax,
) -> anyhow::Result<()> {
    let mut functions = Functions::new();
    let mut data_stack = DataStack::new();

    evaluate_syntax_tree(start, &syntax, &mut functions, &mut data_stack)?;

    Ok(())
}

fn evaluate_syntax_tree(
    start: Option<SyntaxHandle>,
    syntax: &Syntax,
    functions: &mut Functions,
    data_stack: &mut DataStack,
) -> anyhow::Result<()> {
    let mut next_handle = start;

    while let Some(handle) = next_handle {
        let fragment = syntax.get(handle);

        evaluate_syntax_element(
            syntax,
            fragment.payload,
            functions,
            data_stack,
        )?;

        next_handle = fragment.next;
    }

    Ok(())
}

fn evaluate_syntax_element(
    syntax: &Syntax,
    syntax_element: SyntaxElement,
    functions: &mut Functions,
    data_stack: &mut DataStack,
) -> anyhow::Result<()> {
    match syntax_element {
        SyntaxElement::FnRef(fn_ref) => match functions.resolve(&fn_ref)? {
            Function::Intrinsic(intrinsic) => intrinsic(functions, data_stack)?,
            Function::UserDefined { body } => {
                evaluate_syntax_tree(
                    body.0.clone().first,
                    syntax,
                    functions,
                    data_stack,
                )?;
            }
        },
        SyntaxElement::Value(value) => {
            data_stack.push(value);
        }
    }

    Ok(())
}
