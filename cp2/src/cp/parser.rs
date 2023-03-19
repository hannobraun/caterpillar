use super::tokenizer::{ExpectedToken, NoMoreTokens, Token, Tokens};

#[derive(Debug)]
pub struct Expressions(pub Vec<Expression>);

#[derive(Debug)]
pub enum Expression {
    /// Binds values from the stack to provided names
    Binding(Vec<String>),

    /// A block of code that is lazily evaluated
    Block(Vec<Expression>),

    /// A word refers to a function or variable
    Word(String),
}

pub fn parse(mut tokens: Tokens) -> Result<Expressions, Error> {
    let mut expressions = Vec::new();

    while let Ok(token) = tokens.next() {
        let expression = parse_expression(token, &mut tokens)?;
        expressions.push(expression);
    }

    Ok(Expressions(expressions))
}

fn parse_expression(
    token: Token,
    tokens: &mut Tokens,
) -> Result<Expression, Error> {
    match token {
        Token::BindingOperator => parse_binding(tokens),
        Token::CurlyBracketOpen => parse_block(tokens),
        Token::Ident(ident) => Ok(Expression::Word(ident)),
        token => Err(Error::UnexpectedToken(token)),
    }
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

fn parse_block(tokens: &mut Tokens) -> Result<Expression, Error> {
    let mut expressions = Vec::new();

    loop {
        let expression = match tokens.next()? {
            Token::CurlyBracketClose => break,
            token => parse_expression(token, tokens)?,
        };

        expressions.push(expression);
    }

    Ok(Expression::Block(expressions))
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
