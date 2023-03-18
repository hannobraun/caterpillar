use super::tokenizer::{Token, Tokens};

pub struct Expressions(pub Vec<Expression>);

pub enum Expression {
    /// A word refers to a function or variable
    Word(String),
}

pub fn parse(tokens: Tokens) -> Result<Expressions, Error> {
    let expressions = tokens
        .0
        .into_iter()
        .map(|token| match token {
            Token::Word(word) => Expression::Word(word),
        })
        .collect();
    Ok(Expressions(expressions))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}
