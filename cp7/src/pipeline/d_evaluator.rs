use crate::{
    data_stack::DataStack,
    functions::{Function, Functions},
};

use super::c_parser::{SyntaxElement, SyntaxTree};

pub fn evaluate(syntax_tree: SyntaxTree) -> anyhow::Result<()> {
    let mut functions = Functions::new();
    let mut data_stack = DataStack::new();

    for syntax_element in syntax_tree.elements {
        evaluate_syntax_element(
            syntax_element,
            &mut functions,
            &mut data_stack,
        )?;
    }

    Ok(())
}

fn evaluate_syntax_element(
    syntax_element: SyntaxElement,
    functions: &mut Functions,
    data_stack: &mut DataStack,
) -> anyhow::Result<()> {
    match syntax_element {
        SyntaxElement::FnRef(fn_ref) => match functions.resolve(&fn_ref)? {
            Function::Intrinsic(intrinsic) => intrinsic(functions, data_stack)?,
            Function::UserDefined { body } => {
                eprintln!("User-defined function: {body:?}");
                panic!("Evaluating user-defined function not supported");
            }
        },
        SyntaxElement::Value(value) => {
            data_stack.push(value);
        }
    }

    Ok(())
}
