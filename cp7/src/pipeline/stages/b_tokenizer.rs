use std::collections::VecDeque;

use crate::pipeline::concepts::tokens::{Token, Tokens};

pub fn tokenize(code: &str) -> Tokens {
    let mut tokens = VecDeque::new();

    for token in code.split_whitespace() {
        let token = tokenize_token(token);
        tokens.push_back(token);
    }

    Tokens::from(tokens)
}

fn tokenize_token(token: &str) -> Token {
    match token {
        "{" => return Token::CurlyBracketOpen,
        "}" => return Token::CurlyBracketClose,
        _ => {}
    }

    if let Some(("", symbol)) = token.split_once(':') {
        return Token::Symbol(symbol.into());
    }

    if let Ok(number) = token.parse::<i64>() {
        return Token::Number(number);
    }

    Token::Word(token.into())
}
