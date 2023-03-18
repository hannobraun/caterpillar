use std::collections::VecDeque;

pub struct Tokens(pub VecDeque<Token>);

impl Tokens {
    pub fn next(&mut self) -> Result<Token, NoMoreTokens> {
        self.0.pop_front().ok_or(NoMoreTokens)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("No more tokens")]
pub struct NoMoreTokens;

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
