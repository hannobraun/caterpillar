use crate::{
    data_stack::DataStack,
    functions::{Function, Functions},
    syntax::{SyntaxElement, SyntaxTree},
};

pub fn evaluate(syntax_tree: SyntaxTree) -> anyhow::Result<()> {
    let mut functions = Functions::new();
    let mut data_stack = DataStack::new();

    evaluate_syntax_tree(syntax_tree, &mut functions, &mut data_stack)?;

    Ok(())
}

fn evaluate_syntax_tree(
    syntax_tree: SyntaxTree,
    functions: &mut Functions,
    data_stack: &mut DataStack,
) -> anyhow::Result<()> {
    for syntax_element in syntax_tree.elements {
        evaluate_syntax_element(syntax_element, functions, data_stack)?;
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
                evaluate_syntax_tree(body.0.clone(), functions, data_stack)?;
            }
        },
        SyntaxElement::Value(value) => {
            data_stack.push(value);
        }
    }

    Ok(())
}
