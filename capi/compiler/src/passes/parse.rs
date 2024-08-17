use std::collections::{BTreeSet, VecDeque};

use crate::syntax::{Function, Script};

use super::tokenize::Token;

pub fn parse(tokens: Vec<Token>) -> Script {
    let mut tokens = Tokens {
        inner: tokens.into(),
    };
    let mut script = Script::default();

    while let Some(function) = parse_named_function(&mut tokens) {
        script.functions.push(function);
    }

    script
}

fn parse_named_function(tokens: &mut Tokens) -> Option<Function> {
    let name = loop {
        if let Some(Token::Comment { .. }) = tokens.inner.front() {
            // Comments in the top-level context are currently ignored.
            tokens.inner.pop_front();
            continue;
        }

        let token = tokens.inner.pop_front()?;

        match token {
            Token::FunctionName { name } => {
                break name;
            }
            token => {
                eprintln!("Unexpected token: {token:?}");
                return None;
            }
        }
    };

    parse_function(tokens);

    Some(Function {
        name: Some(name),
        branches: Vec::new(),
        environment: BTreeSet::new(),
    })
}

fn parse_function(tokens: &mut Tokens) -> Option<Function> {
    match tokens.inner.pop_front()? {
        Token::FunctionStart => {}
        token => {
            eprintln!("Unexpected token: {token:?}");
            return None;
        }
    }

    while let Some(token) = tokens.inner.front() {
        match token {
            Token::FunctionStart => {
                parse_function(tokens);
            }
            Token::FunctionEnd => {
                tokens.inner.pop_front();
                break;
            }
            _ => {
                tokens.inner.pop_front();
            }
        }
    }

    Some(Function::default())
}

struct Tokens {
    inner: VecDeque<Token>,
}
