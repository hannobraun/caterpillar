use std::collections::VecDeque;

pub struct Tokens(pub VecDeque<Token>);

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    BindingOperator,
    RoundBracketOpen,
    RoundBracketClose,
    Word(String),
}

pub fn tokenize(code: &str) -> Tokens {
    let tokens = code
        .split_whitespace()
        .map(|token| match token {
            "=>" => Token::BindingOperator,
            "(" => Token::RoundBracketOpen,
            ")" => Token::RoundBracketClose,
            token => Token::Word(token.into()),
        })
        .collect();
    Tokens(tokens)
}
