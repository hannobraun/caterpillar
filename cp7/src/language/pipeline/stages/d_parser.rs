use std::iter;

use crate::language::{
    syntax::{
        FragmentAddress, FragmentId, Syntax, SyntaxElement, SyntaxFragment,
    },
    tokens::{token, Token, TokensLeftToRight},
    value::{self, Value},
};

pub fn parse(
    tokens: TokensLeftToRight,
    syntax: &mut Syntax,
) -> ParserResult<ParserOutput> {
    let mut tokens = tokens.peekable();
    let start = parse_fragment(None, &mut tokens, syntax)?;
    Ok(ParserOutput { start })
}

fn parse_fragment(
    terminator: Option<Token>,
    tokens: &mut TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<Option<FragmentId>> {
    let next_token = match tokens.peek() {
        Some(token) => *token,
        None => {
            if terminator.is_none() {
                // If there is no terminator, then not having any more tokens is
                // not an error condition. We've simply reached the end of the
                // input.
                return Ok(None);
            }

            return Err(NoMoreTokens.into());
        }
    };

    let payload = match next_token {
        Token::CurlyBracketOpen => {
            let start = parse_block(tokens, syntax)?;
            let block = value::Block { start };
            SyntaxElement::Value(block.into())
        }
        Token::Word(_) => {
            let word = parse_word(tokens)?;
            SyntaxElement::Word(word)
        }
        Token::Number(_) => {
            let number = parse_number(tokens)?;
            SyntaxElement::Value(Value::Number(number))
        }
        Token::Symbol(_) => {
            let symbol = parse_symbol(tokens)?;
            SyntaxElement::Value(value::Symbol(symbol).into())
        }
        token => {
            if Some(token) == terminator.as_ref() {
                // Only peeked before; still need to consume.
                tokens.next().unwrap();
                return Ok(None);
            }

            return Err(ParserError::UnexpectedToken {
                actual: token.clone(),
            });
        }
    };

    let next = parse_fragment(terminator, tokens, syntax)?;
    let handle =
        syntax.add(SyntaxFragment::new(payload, FragmentAddress { next }));

    Ok(Some(handle))
}

fn parse_block(
    tokens: &mut TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<Option<FragmentId>> {
    expect::<token::CurlyBracketOpen>(tokens)?;

    let handle =
        parse_fragment(Some(Token::CurlyBracketClose), tokens, syntax)?;

    Ok(handle)
}

fn parse_word(tokens: &mut TokenIter) -> ParserResult<String> {
    let payload = expect::<token::Word>(tokens)?;
    Ok(payload.0)
}

fn parse_number(tokens: &mut TokenIter) -> ParserResult<i64> {
    let payload = expect::<token::Number>(tokens)?;
    Ok(payload.0)
}

fn parse_symbol(tokens: &mut TokenIter) -> ParserResult<String> {
    let payload = expect::<token::Symbol>(tokens)?;
    Ok(payload.0)
}

fn expect<T>(tokens: &mut TokenIter) -> ParserResult<T>
where
    T: TryFrom<Token, Error = Token>,
{
    let token = tokens.next().ok_or(NoMoreTokens)?;

    let payload = token
        .clone()
        .try_into()
        .map_err(|token| ParserError::UnexpectedToken { actual: token })?;

    Ok(payload)
}

pub type TokenIter<'r> = iter::Peekable<TokensLeftToRight<'r>>;

pub type ParserResult<T> = Result<T, ParserError>;

pub struct ParserOutput {
    pub start: Option<FragmentId>,
}

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
