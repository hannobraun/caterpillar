use std::collections::{BTreeSet, VecDeque};

use crate::syntax::{Branch, Function, Script};

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
        if let Some(Token::Comment { .. }) = tokens.peek() {
            // Comments in the top-level context are currently ignored.
            tokens.take();
            continue;
        }

        match tokens.take()? {
            Token::FunctionName { name } => {
                break name;
            }
            token => {
                panic!("Unexpected token: {token:?}");
            }
        }
    };

    let mut function = parse_function(tokens)?;
    function.name = Some(name);

    Some(function)
}

fn parse_function(tokens: &mut Tokens) -> Option<Function> {
    match tokens.take()? {
        Token::FunctionStart => {}
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    while parse_branch(tokens).is_some() {}

    Some(Function {
        name: None,
        branches: Vec::new(),
        environment: BTreeSet::new(),
    })
}

fn parse_branch(tokens: &mut Tokens) -> Option<Branch> {
    while let Some(token) = tokens.peek() {
        match token {
            Token::FunctionStart => {
                parse_function(tokens);
            }
            Token::FunctionEnd => {
                tokens.take();
                break;
            }
            _ => {
                tokens.take();
            }
        }
    }

    None
}

struct Tokens {
    inner: VecDeque<Token>,
}

impl Tokens {
    pub fn peek(&self) -> Option<&Token> {
        self.inner.front()
    }

    pub fn take(&mut self) -> Option<Token> {
        self.inner.pop_front()
    }
}
