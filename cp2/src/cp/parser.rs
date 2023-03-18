use super::tokenizer::{ExpectedToken, NoMoreTokens, Token, Tokens};

pub struct Expressions(pub Vec<Expression>);

pub enum Expression {
    /// Binds values from the stack to provided names
    Binding(Vec<String>),

    /// A word refers to a function or variable
    Word(String),
}

pub fn parse(mut tokens: Tokens) -> Result<Expressions, Error> {
    let mut expressions = Vec::new();

    while let Ok(token) = tokens.next() {
        let expression = match token {
            Token::BindingOperator => parse_binding(&mut tokens)?,
            Token::Ident(ident) => Expression::Word(ident),
            token => return Err(Error::UnexpectedToken(token)),
        };

        expressions.push(expression);
    }

    Ok(Expressions(expressions))
}

fn parse_binding(tokens: &mut Tokens) -> Result<Expression, Error> {
    let mut binding_names = Vec::new();

    tokens.expect(Token::RoundBracketOpen)?;

    loop {
        match tokens.next()? {
            Token::Ident(ident) => binding_names.push(ident),
            Token::RoundBracketClose => break,
            token => return Err(Error::UnexpectedToken(token)),
        }
    }

    Ok(Expression::Binding(binding_names))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Expected more tokens")]
    ExpectedMoreTokens(#[from] NoMoreTokens),

    #[error(transparent)]
    ExpectedToken(#[from] ExpectedToken),

    #[error("Unexpected token: `{0:?}`")]
    UnexpectedToken(Token),
}
