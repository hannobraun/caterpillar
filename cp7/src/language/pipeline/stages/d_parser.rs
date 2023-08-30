use std::iter;

use crate::language::{
    syntax::{
        FragmentAddress, FragmentId, Syntax, SyntaxElement, SyntaxFragment,
    },
    tokens::{token, Token, TokenAddress, Tokens, TokensLeftToRight},
    value::{self, Value},
};

pub fn parse(
    tokens: &Tokens,
    token_iter: TokensLeftToRight,
    syntax: &mut Syntax,
) -> ParserResult<ParserOutput> {
    let mut token_iter = token_iter.peekable();

    let (start, _) = parse_fragment(None, tokens, &mut token_iter, syntax)?;

    Ok(ParserOutput { start })
}

fn parse_fragment(
    terminator: Option<Token>,
    tokens: &Tokens,
    token_iter: &mut TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<(Option<FragmentId>, Option<TokenAddress>)> {
    let next_token = match token_iter.peek() {
        Some(token) => tokens.by_address.get(token).unwrap(),
        None => {
            if terminator.is_none() {
                // If there is no terminator, then not having any more tokens is
                // not an error condition. We've simply reached the end of the
                // input.
                return Ok((None, None));
            }

            return Err(NoMoreTokens.into());
        }
    };

    let payload = match next_token {
        Token::CurlyBracketOpen => {
            let start = parse_block(tokens, token_iter, syntax)?;
            let block = value::Block { start };
            SyntaxElement::Value(block.into())
        }
        Token::Word(_) => {
            let word = parse_word(tokens, token_iter)?;
            SyntaxElement::Word(word)
        }
        Token::Number(_) => {
            let number = parse_number(tokens, token_iter)?;
            SyntaxElement::Value(Value::Number(number))
        }
        Token::Symbol(_) => {
            let symbol = parse_symbol(tokens, token_iter)?;
            SyntaxElement::Value(value::Symbol(symbol).into())
        }
        token => {
            if Some(token) == terminator.as_ref() {
                // Only peeked before; still need to consume.
                let token = token_iter.next().unwrap();
                return Ok((None, Some(token)));
            }

            return Err(ParserError::UnexpectedToken {
                actual: token.clone(),
            });
        }
    };

    let (next, hash) = parse_fragment(terminator, tokens, token_iter, syntax)?;
    let handle =
        syntax.add(SyntaxFragment::new(payload, FragmentAddress { next }));

    Ok((Some(handle), hash))
}

fn parse_block(
    tokens: &Tokens,
    token_iter: &mut TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<Option<FragmentId>> {
    expect::<token::CurlyBracketOpen>(tokens, token_iter)?;

    let (handle, _) = parse_fragment(
        Some(Token::CurlyBracketClose),
        tokens,
        token_iter,
        syntax,
    )?;

    Ok(handle)
}

fn parse_word(
    tokens: &Tokens,
    token_iter: &mut TokenIter,
) -> ParserResult<String> {
    let (payload, _) = expect::<token::Word>(tokens, token_iter)?;
    Ok(payload.0)
}

fn parse_number(
    tokens: &Tokens,
    token_iter: &mut TokenIter,
) -> ParserResult<i64> {
    let (payload, _) = expect::<token::Number>(tokens, token_iter)?;
    Ok(payload.0)
}

fn parse_symbol(
    tokens: &Tokens,
    token_iter: &mut TokenIter,
) -> ParserResult<String> {
    let (payload, _) = expect::<token::Symbol>(tokens, token_iter)?;
    Ok(payload.0)
}

fn expect<T>(
    tokens: &Tokens,
    token_iter: &mut TokenIter,
) -> ParserResult<(T, TokenAddress)>
where
    T: TryFrom<Token, Error = Token>,
{
    let token = token_iter.next().ok_or(NoMoreTokens)?;

    let payload = tokens
        .by_address
        .get(&token)
        .unwrap()
        .clone()
        .try_into()
        .map_err(|token| ParserError::UnexpectedToken { actual: token })?;

    Ok((payload, token))
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
