use std::{fs::File, io::Read};

use clap::Parser;

mod data_stack;
mod tokenizer;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let path = format!("cp7/examples/{}.cp", args.example);

    let mut code = String::new();
    File::open(path)?.read_to_string(&mut code)?;

    let mut data_stack = data_stack::DataStack::new();

    let tokens = tokenizer::tokenize(&code);

    for token in tokens {
        match token {
            tokenizer::Token::FnRef(fn_ref) => match fn_ref.as_str() {
                "1" => data_stack.push(1),
                "2" => data_stack.push(2),
                "+" => {
                    let b = data_stack.pop().unwrap();
                    let a = data_stack.pop().unwrap();
                    data_stack.push(a + b);
                }
                "print_line" => {
                    let value = data_stack.pop().unwrap();
                    println!("{value}");
                }
                token => {
                    eprintln!("Unexpected token: {token}");
                    break;
                }
            },
        }
    }

    Ok(())
}

#[derive(clap::Parser)]
pub struct Args {
    pub example: String,
}
