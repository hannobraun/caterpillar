use std::iter;

use crate::language::{
    syntax::{
        Syntax, SyntaxElement, SyntaxFragment, SyntaxHandle, SyntaxToTokens,
        TokenRange,
    },
    tokens::{token, AddressedToken, Token, TokensLeftToRight},
    value::{self, Value},
};

pub fn parse(
    tokens: TokensLeftToRight,
    syntax: &mut Syntax,
) -> ParserResult<ParserOutput> {
    let mut tokens = tokens.peekable();
    let mut syntax_to_tokens = SyntaxToTokens::new();

    let (start, _) =
        parse_fragment(None, &mut tokens, syntax, &mut syntax_to_tokens)?;

    Ok(ParserOutput {
        start,
        syntax_to_tokens,
    })
}

fn parse_fragment(
    terminator: Option<Token>,
    token_iter: &mut TokenIter,
    syntax: &mut Syntax,
    syntax_to_tokens: &mut SyntaxToTokens,
) -> ParserResult<(Option<SyntaxHandle>, Option<AddressedToken>)> {
    let next_token = match token_iter.peek() {
        Some(token) => &token.token,
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

    let (syntax_element, token_range) = match next_token {
        Token::CurlyBracketOpen => {
            let (start, token_range) =
                parse_block(token_iter, syntax, syntax_to_tokens)?;

            let block = value::Block { start };
            let block = SyntaxElement::Value(block.into());

            (block, token_range)
        }
        Token::Word(_) => {
            let (word, token_range) = parse_word(token_iter)?;
            let word = SyntaxElement::Word(word);
            (word, token_range)
        }
        Token::Number(_) => {
            let (number, token_range) = parse_number(token_iter)?;
            let number = SyntaxElement::Value(Value::Number(number));
            (number, token_range)
        }
        Token::Symbol(_) => {
            let (symbol, token_range) = parse_symbol(token_iter)?;
            let symbol = SyntaxElement::Value(value::Symbol(symbol).into());
            (symbol, token_range)
        }
        token => {
            if Some(token) == terminator.as_ref() {
                // Only peeked before; still need to consume.
                let token = token_iter.next().unwrap();
                return Ok((None, Some(token.clone())));
            }

            return Err(ParserError::UnexpectedToken {
                actual: token.clone(),
            });
        }
    };

    let (next, hash) =
        parse_fragment(terminator, token_iter, syntax, syntax_to_tokens)?;
    let handle = syntax.add(SyntaxFragment {
        payload: syntax_element,
        next,
    });
    syntax_to_tokens.insert(handle, token_range);

    Ok((Some(handle), hash))
}

fn parse_block(
    token_iter: &mut TokenIter,
    syntax: &mut Syntax,
    syntax_to_tokens: &mut SyntaxToTokens,
) -> ParserResult<(Option<SyntaxHandle>, TokenRange)> {
    let (_, start) = expect::<token::CurlyBracketOpen>(token_iter)?;

    let (handle, end) = parse_fragment(
        Some(Token::CurlyBracketClose),
        token_iter,
        syntax,
        syntax_to_tokens,
    )?;

    let end = end.unwrap();
    let range = TokenRange { start, end };

    Ok((handle, range))
}

fn parse_word(
    token_iter: &mut TokenIter,
) -> ParserResult<(String, TokenRange)> {
    let (payload, token) = expect::<token::Word>(token_iter)?;
    Ok((payload.0, TokenRange::one(token)))
}

fn parse_number(tokens: &mut TokenIter) -> ParserResult<(i64, TokenRange)> {
    let (payload, token) = expect::<token::Number>(tokens)?;
    Ok((payload.0, TokenRange::one(token)))
}

fn parse_symbol(tokens: &mut TokenIter) -> ParserResult<(String, TokenRange)> {
    let (payload, token) = expect::<token::Symbol>(tokens)?;
    Ok((payload.0, TokenRange::one(token)))
}

fn expect<T>(tokens: &mut TokenIter) -> ParserResult<(T, AddressedToken)>
where
    T: TryFrom<Token, Error = Token>,
{
    let token = tokens.next().ok_or(NoMoreTokens)?;

    let payload = token
        .token
        .clone()
        .try_into()
        .map_err(|token| ParserError::UnexpectedToken { actual: token })?;

    Ok((payload, token.clone()))
}

pub type TokenIter<'r> = iter::Peekable<TokensLeftToRight<'r>>;

pub type ParserResult<T> = Result<T, ParserError>;

pub struct ParserOutput {
    pub start: Option<SyntaxHandle>,
    pub syntax_to_tokens: SyntaxToTokens,
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
