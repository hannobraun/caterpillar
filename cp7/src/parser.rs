use std::collections::VecDeque;

use enum_tag::EnumTag;

use crate::tokenizer::{Token, TokenTag};

pub fn parse(mut tokens: VecDeque<Token>) -> ParserResult<Vec<SyntaxElement>> {
    let mut syntax_elements = Vec::new();

    while let Some(token) = tokens.front() {
        let syntax_element = parse_syntax_element(token.tag(), &mut tokens)?;
        syntax_elements.push(syntax_element);
    }

    Ok(syntax_elements)
}

fn parse_syntax_element(
    next_token: TokenTag,
    tokens: &mut VecDeque<Token>,
) -> ParserResult<SyntaxElement> {
    match next_token {
        TokenTag::CurlyBracketOpen => panic!("Parsing block not supported"),
        TokenTag::FnRef => {
            let fn_ref = parse_fn_ref(tokens)?;
            Ok(SyntaxElement::FnRef(fn_ref))
        }
        TokenTag::Symbol => {
            let symbol = parse_symbol(tokens)?;
            Ok(SyntaxElement::Symbol(symbol))
        }
        token => panic!("Unexpected token: {token:?}"),
    }
}

fn parse_fn_ref(tokens: &mut VecDeque<Token>) -> ParserResult<String> {
    match tokens.pop_front().unwrap() {
        Token::FnRef(fn_ref) => Ok(fn_ref),
        token => panic!("Unexpected token: {token:?}"),
    }
}

fn parse_symbol(tokens: &mut VecDeque<Token>) -> ParserResult<String> {
    match tokens.pop_front().unwrap() {
        Token::Symbol(symbol) => Ok(symbol),
        token => panic!("Unexpected token: {token:?}"),
    }
}

pub enum SyntaxElement {
    FnRef(String),
    Symbol(String),
}

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {}
