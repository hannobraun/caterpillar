use std::collections::VecDeque;

use enum_tag::EnumTag;

use crate::tokenizer::{Token, TokenTag};

pub fn parse(mut tokens: VecDeque<Token>) -> Vec<SyntaxElement> {
    let mut syntax_elements = Vec::new();

    while let Some(token) = tokens.front() {
        let syntax_element = parse_syntax_element(token.tag(), &mut tokens);
        syntax_elements.push(syntax_element);
    }

    syntax_elements
}

fn parse_syntax_element(
    next_token: TokenTag,
    tokens: &mut VecDeque<Token>,
) -> SyntaxElement {
    match next_token {
        TokenTag::CurlyBracketOpen => panic!("Parsing block not supported"),
        TokenTag::FnRef => {
            let fn_ref = parse_fn_ref(tokens);
            SyntaxElement::FnRef(fn_ref)
        }
        TokenTag::Symbol => {
            let symbol = parse_symbol(tokens);
            SyntaxElement::Symbol(symbol)
        }
        token => panic!("Unexpected token: {token:?}"),
    }
}

fn parse_fn_ref(tokens: &mut VecDeque<Token>) -> String {
    match tokens.pop_front().unwrap() {
        Token::FnRef(fn_ref) => fn_ref,
        token => panic!("Unexpected token: {token:?}"),
    }
}

fn parse_symbol(tokens: &mut VecDeque<Token>) -> String {
    match tokens.pop_front().unwrap() {
        Token::Symbol(symbol) => symbol,
        token => panic!("Unexpected token: {token:?}"),
    }
}

pub enum SyntaxElement {
    FnRef(String),
    Symbol(String),
}
