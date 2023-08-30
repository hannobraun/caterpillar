use std::iter;

use crate::language::{
    syntax::{
        FragmentAddress, FragmentId, Syntax, SyntaxElement, SyntaxFragment,
    },
    tokens::{token, Token, Tokens, TokensLeftToRight},
    value::{self, Value},
};

pub fn parse(
    _: &Tokens,
    token_iter: TokensLeftToRight,
    syntax: &mut Syntax,
) -> ParserResult<ParserOutput> {
    let mut token_iter = token_iter.peekable();
    let start = parse_fragment(None, &mut token_iter, syntax)?;
    Ok(ParserOutput { start })
}

fn parse_fragment(
    terminator: Option<Token>,
    token_iter: &mut TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<Option<FragmentId>> {
    let next_token = match token_iter.peek() {
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
            let start = parse_block(token_iter, syntax)?;
            let block = value::Block { start };
            SyntaxElement::Value(block.into())
        }
        Token::Word(_) => {
            let word = parse_word(token_iter)?;
            SyntaxElement::Word(word)
        }
        Token::Number(_) => {
            let number = parse_number(token_iter)?;
            SyntaxElement::Value(Value::Number(number))
        }
        Token::Symbol(_) => {
            let symbol = parse_symbol(token_iter)?;
            SyntaxElement::Value(value::Symbol(symbol).into())
        }
        token => {
            if Some(token) == terminator.as_ref() {
                // Only peeked before; still need to consume.
                token_iter.next().unwrap();
                return Ok(None);
            }

            return Err(ParserError::UnexpectedToken {
                actual: token.clone(),
            });
        }
    };

    let next = parse_fragment(terminator, token_iter, syntax)?;
    let handle =
        syntax.add(SyntaxFragment::new(payload, FragmentAddress { next }));

    Ok(Some(handle))
}

fn parse_block(
    token_iter: &mut TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<Option<FragmentId>> {
    expect::<token::CurlyBracketOpen>(token_iter)?;

    let handle =
        parse_fragment(Some(Token::CurlyBracketClose), token_iter, syntax)?;

    Ok(handle)
}

fn parse_word(token_iter: &mut TokenIter) -> ParserResult<String> {
    let payload = expect::<token::Word>(token_iter)?;
    Ok(payload.0)
}

fn parse_number(token_iter: &mut TokenIter) -> ParserResult<i64> {
    let payload = expect::<token::Number>(token_iter)?;
    Ok(payload.0)
}

fn parse_symbol(token_iter: &mut TokenIter) -> ParserResult<String> {
    let payload = expect::<token::Symbol>(token_iter)?;
    Ok(payload.0)
}

fn expect<T>(token_iter: &mut TokenIter) -> ParserResult<T>
where
    T: TryFrom<Token, Error = Token>,
{
    let token = token_iter.next().ok_or(NoMoreTokens)?;

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
