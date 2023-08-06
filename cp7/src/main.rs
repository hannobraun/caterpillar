use data_stack::value;

mod args;
mod data_stack;
mod pipeline;

fn main() -> anyhow::Result<()> {
    let example = args::example()?;

    let code = pipeline::a_loader::load(example)?;
    let tokens = pipeline::b_tokenizer::tokenize(&code);
    let syntax_tree = pipeline::parser::parse(tokens)?;

    let mut data_stack = data_stack::DataStack::new();

    for syntax_element in syntax_tree.elements {
        match syntax_element {
            pipeline::parser::SyntaxElement::FnRef(fn_ref) => {
                match fn_ref.as_str() {
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
                }
            }
            pipeline::parser::SyntaxElement::Value(value) => {
                data_stack.push(value);
            }
        }
    }

    Ok(())
}
