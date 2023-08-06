use crate::{
    data_stack::DataStack,
    functions::{Function, Functions},
    syntax::{Syntax, SyntaxElement, SyntaxTree},
};

pub fn evaluate(syntax: Syntax, syntax_tree: SyntaxTree) -> anyhow::Result<()> {
    let mut functions = Functions::new();
    let mut data_stack = DataStack::new();

    evaluate_syntax_tree(
        &syntax,
        syntax_tree,
        &mut functions,
        &mut data_stack,
    )?;

    Ok(())
}

fn evaluate_syntax_tree(
    syntax: &Syntax,
    syntax_tree: SyntaxTree,
    functions: &mut Functions,
    data_stack: &mut DataStack,
) -> anyhow::Result<()> {
    for fragment in syntax_tree.elements {
        evaluate_syntax_element(
            syntax,
            fragment.payload,
            functions,
            data_stack,
        )?;
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
                    syntax,
                    body.0.clone(),
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
