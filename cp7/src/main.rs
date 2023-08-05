use std::{
    fs::{self, File},
    io::Read,
};

use clap::Parser;

mod data_stack;
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

    for token in tokens {
        match token {
            tokenizer::Token::CurlyBracketOpen => {
                eprintln!("{{");
            }
            tokenizer::Token::CurlyBracketClose => {
                eprintln!("}}");
            }
            tokenizer::Token::FnRef(fn_ref) => match fn_ref.as_str() {
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
                    eprintln!("Unexpected token: {token}");
                    break;
                }
            },
            tokenizer::Token::Symbol(symbol) => {
                println!("Symbol: {symbol}");
            }
        }
    }

    Ok(())
}

#[derive(clap::Parser)]
pub struct Args {
    pub example: Option<String>,
}
