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
            Token::BindingOperator => Err(Error::UnexpectedToken(token)),
            Token::Word(word) => Ok(Expression::Word(word)),
        })
        .collect::<Result<_, _>>()?;
    Ok(Expressions(expressions))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected token: `{0:?}`")]
    UnexpectedToken(Token),
}
