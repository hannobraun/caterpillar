use crate::{
    data_stack::DataStack,
    functions::{Function, Functions},
};

use super::c_parser::{SyntaxElement, SyntaxTree};

pub fn evaluate(syntax_tree: SyntaxTree) -> anyhow::Result<()> {
    let mut functions = Functions::new();
    let mut data_stack = DataStack::new();

    for syntax_element in syntax_tree.elements {
        match syntax_element {
            SyntaxElement::FnRef(fn_ref) => {
                let Function::Intrinsic(intrinsic) =
                    functions.resolve(&fn_ref)?;
                intrinsic(&mut functions, &mut data_stack)?;
            }
            SyntaxElement::Value(value) => {
                data_stack.push(value);
            }
        }
    }

    Ok(())
}
