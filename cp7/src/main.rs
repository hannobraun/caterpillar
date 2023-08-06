use std::{
    fs::{self, File},
    io::Read,
};

use clap::Parser;

mod data_stack;
mod parser;
mod tokenizer;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let example_dir = "cp7/examples";
    let path = if let Some(example) = args.example {
        format!("cp7/examples/{example}.cp")
    } else {
        eprintln!("Need to specify example. Available examples:");

        for dir_entry in fs::read_dir(example_dir)? {
            let path = dir_entry?.path();
            let example = path.file_stem().unwrap().to_string_lossy();
            eprintln!("- {example}");
        }

        return Ok(());
    };

    let mut code = String::new();
    File::open(path)?.read_to_string(&mut code)?;

    let mut data_stack = data_stack::DataStack::new();

    let tokens = tokenizer::tokenize(&code);
    let syntax_tree = parser::parse(tokens)?;

    for syntax_element in syntax_tree {
        match syntax_element {
            parser::SyntaxElement::Block(block) => {
                eprintln!("Block: {block:?}")
            }
            parser::SyntaxElement::FnRef(fn_ref) => match fn_ref.as_str() {
                "1" => data_stack.push(1),
                "2" => data_stack.push(2),
                "+" => {
                    let b = data_stack.pop_number()?;
                    let a = data_stack.pop_number()?;
                    data_stack.push(a + b);
                }
                "print_line" => {
                    let value = data_stack.pop_any()?;
                    println!("{value}");
                }
                token => {
                    eprintln!("Unknown function: `{token}`");
                    break;
                }
            },
            parser::SyntaxElement::Symbol(symbol) => {
                eprintln!("Symbol: {symbol}")
            }
        }
    }

    Ok(())
}

#[derive(clap::Parser)]
pub struct Args {
    pub example: Option<String>,
}
