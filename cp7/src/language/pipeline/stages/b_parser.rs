use std::{iter, vec};

use crate::language::repr::tokens::{token, Token};

pub fn parse_word(tokens: &mut Tokens) -> ParserResult<String> {
    let token = expect::<token::Word>(tokens)?;
    Ok(token.0)
}

pub fn parse_number(tokens: &mut Tokens) -> ParserResult<i64> {
    let token = expect::<token::Number>(tokens)?;
    Ok(token.0)
}

pub fn parse_symbol(tokens: &mut Tokens) -> ParserResult<String> {
    let token = expect::<token::Symbol>(tokens)?;
    Ok(token.0)
}

pub fn expect<T>(tokens: &mut Tokens) -> ParserResult<T>
where
    T: TryFrom<Token, Error = Token>,
{
    let token = tokens.next().ok_or(NoMoreTokens)?;

    let token = token
        .try_into()
        .map_err(|token| ParserError::UnexpectedToken { actual: token })?;

    Ok(token)
}

pub type Tokens<'r> = iter::Peekable<vec::IntoIter<Token>>;

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Expected more tokens")]
    ExpectedMoreTokens(#[from] NoMoreTokens),

    #[error("Unexpected token: {actual:?}")]
    UnexpectedToken { actual: Token },
}

#[derive(Debug, thiserror::Error)]
#[error("No more tokens")]
pub struct NoMoreTokens;
