use std::collections::{BTreeSet, VecDeque};

use crate::syntax::{Branch, Expression, Function, Pattern, Script};

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

    let mut branches = Vec::new();
    while let Some(branch) = parse_branch(tokens) {
        branches.push(branch);
    }

    match tokens.take()? {
        Token::FunctionEnd => {}
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    Some(Function {
        name: None,
        branches,
        environment: BTreeSet::new(),
    })
}

fn parse_branch(tokens: &mut Tokens) -> Option<Branch> {
    match tokens.peek()? {
        Token::BranchHeadBoundary => {
            tokens.take();
        }
        Token::FunctionEnd => {
            return None;
        }
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    let mut parameters = Vec::new();
    while let Some(token) = tokens.take() {
        match token {
            Token::Identifier { name } => {
                parameters.push(Pattern::Identifier { name });
            }
            Token::IntegerLiteral { value } => {
                parameters.push(Pattern::Literal {
                    value: value.into(),
                });
            }
            Token::BranchHeadBoundary => {
                break;
            }
            token => {
                panic!("Unexpected token: {token:?}");
            }
        }
    }

    let mut body = Vec::new();
    while let Some(token) = tokens.peek() {
        match token {
            Token::FunctionStart => {
                parse_function(tokens);
            }
            Token::BranchHeadBoundary | Token::FunctionEnd => {
                break;
            }
            _ => match tokens.take()? {
                Token::Comment { text } => {
                    body.push(Expression::Comment { text });
                }
                Token::Identifier { name } => {
                    body.push(Expression::Identifier {
                        name,
                        target: None,
                        is_known_to_be_in_tail_position: false,
                    });
                }
                Token::IntegerLiteral { value } => {
                    body.push(Expression::Value(value.into()));
                }
                Token::BindingStart | Token::BindingEnd => {}

                token => {
                    panic!("Unexpected token: {token:?}");
                }
            },
        }
    }

    Some(Branch { parameters, body })
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
