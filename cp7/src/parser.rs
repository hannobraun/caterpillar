use std::collections::VecDeque;

use crate::tokenizer::Token;

pub fn parse(mut tokens: VecDeque<Token>) -> Vec<SyntaxElement> {
    let mut syntax_elements = Vec::new();

    while let Some(token) = tokens.front() {
        let syntax_element = match token {
            Token::CurlyBracketOpen => panic!("Parsing block not supported"),
            Token::FnRef(_) => match tokens.pop_front().unwrap() {
                Token::FnRef(fn_ref) => SyntaxElement::FnRef(fn_ref),
                token => panic!("Unexpected token: {token:?}"),
            },
            Token::Symbol(_) => panic!("Parsing symbol not supported"),
            token => panic!("Unexpected token: {token:?}"),
        };

        syntax_elements.push(syntax_element);
    }

    syntax_elements
}

pub enum SyntaxElement {
    FnRef(String),
}
