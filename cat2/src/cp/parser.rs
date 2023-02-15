use super::{Token, Tokens};

pub type Expressions = Vec<Expression>;

pub enum Expression {
    Fn(String),
}

pub fn parse(tokens: Tokens) -> Expressions {
    tokens
        .into_iter()
        .map(|token| match token {
            Token::Fn(name) => Expression::Fn(name),
        })
        .collect()
}
