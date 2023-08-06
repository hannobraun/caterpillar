use anyhow::bail;

use crate::{data_stack::DataStack, functions};

use super::c_parser::{SyntaxElement, SyntaxTree};

pub fn evaluate(syntax_tree: SyntaxTree) -> anyhow::Result<()> {
    let mut data_stack = DataStack::new();

    for syntax_element in syntax_tree.elements {
        match syntax_element {
            SyntaxElement::FnRef(fn_ref) => match fn_ref.as_str() {
                "+" => {
                    functions::add(&mut data_stack)?;
                }
                "print_line" => {
                    functions::print_line(&mut data_stack)?;
                }
                fn_ref => {
                    bail!("Unknown function: `{fn_ref}`");
                }
            },
            SyntaxElement::Value(value) => {
                data_stack.push(value);
            }
        }
    }

    Ok(())
}
