use anyhow::bail;

use crate::data_stack::{value, DataStack, DataStackResult};

use super::c_parser::{SyntaxElement, SyntaxTree};

pub fn evaluate(syntax_tree: SyntaxTree) -> anyhow::Result<()> {
    let mut data_stack = DataStack::new();

    for syntax_element in syntax_tree.elements {
        match syntax_element {
            SyntaxElement::FnRef(fn_ref) => match fn_ref.as_str() {
                "+" => {
                    add(&mut data_stack)?;
                }
                "print_line" => {
                    print_line(&mut data_stack)?;
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

fn add(data_stack: &mut DataStack) -> DataStackResult<()> {
    let b = data_stack.pop_number()?;
    let a = data_stack.pop_number()?;

    data_stack.push(value::Number(a.0 + b.0));

    Ok(())
}

fn print_line(data_stack: &mut DataStack) -> DataStackResult<()> {
    let value = data_stack.pop_any()?;
    println!("{value}");
    Ok(())
}
