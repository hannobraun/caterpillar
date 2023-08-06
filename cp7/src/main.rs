use std::{fs::File, io::Read};

use data_stack::value;

mod args;
mod data_stack;
mod parser;
mod tokenizer;

fn main() -> anyhow::Result<()> {
    let example = args::example()?;

    let mut code = String::new();
    File::open(example)?.read_to_string(&mut code)?;

    let tokens = tokenizer::tokenize(&code);
    let syntax_tree = parser::parse(tokens)?;

    let mut data_stack = data_stack::DataStack::new();

    for syntax_element in syntax_tree.elements {
        match syntax_element {
            parser::SyntaxElement::FnRef(fn_ref) => match fn_ref.as_str() {
                "+" => {
                    let b = data_stack.pop_number()?;
                    let a = data_stack.pop_number()?;
                    data_stack.push(value::Number(a.0 + b.0));
                }
                "print_line" => {
                    let value = data_stack.pop_any()?;
                    println!("{value}");
                }
                fn_ref => {
                    eprintln!("Unknown function: `{fn_ref}`");
                    break;
                }
            },
            parser::SyntaxElement::Value(value) => {
                data_stack.push(value);
            }
        }
    }

    Ok(())
}
