use crate::language::{
    syntax::{Syntax, SyntaxElement, SyntaxFragment, SyntaxHandle},
    tokens::{token, Token, TokenIter},
    value::{self, Value},
};

pub fn parse(
    mut tokens: TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<Option<SyntaxHandle>> {
    let start = parse_fragment(None, &mut tokens, syntax)?;
    Ok(start)
}

fn parse_fragment(
    terminator: Option<Token>,
    tokens: &mut TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<Option<SyntaxHandle>> {
    let next_token = match tokens.peek() {
        Some(token) => token,
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

    let syntax_element = match next_token.token {
        Token::CurlyBracketOpen => {
            let block = parse_block(tokens, syntax)?;
            SyntaxElement::Value(value::Block(block).into())
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
            if Some(&token) == terminator.as_ref() {
                // Only peeked before; still need to consume.
                let _ = tokens.next();
                return Ok(None);
            }

            return Err(ParserError::UnexpectedToken { actual: token });
        }
    };

    let next = parse_fragment(terminator, tokens, syntax)?;
    let handle = syntax.add(SyntaxFragment {
        payload: syntax_element,
        next,
    });

    Ok(Some(handle))
}

fn parse_block(
    tokens: &mut TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<Option<SyntaxHandle>> {
    expect::<token::CurlyBracketOpen>(tokens)?;
    parse_fragment(Some(Token::CurlyBracketClose), tokens, syntax)
}

fn parse_word(tokens: &mut TokenIter) -> ParserResult<String> {
    let (token, _) = expect::<token::Word>(tokens)?;
    Ok(token.0)
}

fn parse_number(tokens: &mut TokenIter) -> ParserResult<i64> {
    let (token, _) = expect::<token::Number>(tokens)?;
    Ok(token.0)
}

fn parse_symbol(tokens: &mut TokenIter) -> ParserResult<String> {
    let (token, _) = expect::<token::Symbol>(tokens)?;
    Ok(token.0)
}

fn expect<T>(tokens: &mut TokenIter) -> ParserResult<(T, blake3::Hash)>
where
    T: TryFrom<Token, Error = Token>,
{
    let token = tokens.next().ok_or(NoMoreTokens)?;

    let hash = {
        let mut hasher = blake3::Hasher::new();

        if let Some(left) = token.left {
            hasher.update(left.hash.as_bytes());
        }
        if let Some(right) = token.right {
            hasher.update(right.hash.as_bytes());
        }

        hasher.finalize()
    };

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
