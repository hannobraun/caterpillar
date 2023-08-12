use crate::language::{
    syntax::{Syntax, SyntaxElement, SyntaxFragment, SyntaxHandle, TokenRange},
    tokens::{token, Token, TokenIter},
    value::{self, Value},
};

pub fn parse(
    mut tokens: TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<Option<SyntaxHandle>> {
    let (start, _) = parse_fragment(None, &mut tokens, syntax)?;
    Ok(start)
}

fn parse_fragment(
    terminator: Option<Token>,
    tokens: &mut TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<(Option<SyntaxHandle>, Option<blake3::Hash>)> {
    let next_token = match tokens.peek() {
        Some(token) => token,
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

    let syntax_element = match next_token.token {
        Token::CurlyBracketOpen => {
            let block = parse_block(tokens, syntax)?;
            SyntaxElement::Value(value::Block(block).into())
        }
        Token::Word(_) => {
            let (word, _) = parse_word(tokens)?;
            SyntaxElement::Word(word)
        }
        Token::Number(_) => {
            let (number, _) = parse_number(tokens)?;
            SyntaxElement::Value(Value::Number(number))
        }
        Token::Symbol(_) => {
            let (symbol, _) = parse_symbol(tokens)?;
            SyntaxElement::Value(value::Symbol(symbol).into())
        }
        token => {
            if Some(&token) == terminator.as_ref() {
                // Only peeked before; still need to consume.
                let token = tokens.next().unwrap();
                return Ok((None, Some(token.hash())));
            }

            return Err(ParserError::UnexpectedToken { actual: token });
        }
    };

    let (next, hash) = parse_fragment(terminator, tokens, syntax)?;
    let handle = syntax.add(SyntaxFragment {
        payload: syntax_element,
        next,
    });

    Ok((Some(handle), hash))
}

fn parse_block(
    tokens: &mut TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<Option<SyntaxHandle>> {
    expect::<token::CurlyBracketOpen>(tokens)?;
    let (handle, _) =
        parse_fragment(Some(Token::CurlyBracketClose), tokens, syntax)?;
    Ok(handle)
}

fn parse_word(tokens: &mut TokenIter) -> ParserResult<(String, TokenRange)> {
    let (token, hash) = expect::<token::Word>(tokens)?;
    Ok((token.0, TokenRange::one(hash)))
}

fn parse_number(tokens: &mut TokenIter) -> ParserResult<(i64, TokenRange)> {
    let (token, hash) = expect::<token::Number>(tokens)?;
    Ok((token.0, TokenRange::one(hash)))
}

fn parse_symbol(tokens: &mut TokenIter) -> ParserResult<(String, TokenRange)> {
    let (token, hash) = expect::<token::Symbol>(tokens)?;
    Ok((token.0, TokenRange::one(hash)))
}

fn expect<T>(tokens: &mut TokenIter) -> ParserResult<(T, blake3::Hash)>
where
    T: TryFrom<Token, Error = Token>,
{
    let token = tokens.next().ok_or(NoMoreTokens)?;

    let hash = token.hash();

    let token = token
        .token
        .try_into()
        .map_err(|token| ParserError::UnexpectedToken { actual: token })?;

    Ok((token, hash))
}

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
