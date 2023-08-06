use crate::{data_stack::DataStack, functions::Functions};

use super::c_parser::{SyntaxElement, SyntaxTree};

pub fn evaluate(syntax_tree: SyntaxTree) -> anyhow::Result<()> {
    let functions = Functions::new();
    let mut data_stack = DataStack::new();

    for syntax_element in syntax_tree.elements {
        match syntax_element {
            SyntaxElement::FnRef(fn_ref) => {
                let intrinsic = functions.resolve(&fn_ref)?;
                intrinsic(&mut data_stack)?;
            }
            SyntaxElement::Value(value) => {
                data_stack.push(value);
            }
        }
    }

    Ok(())
}
