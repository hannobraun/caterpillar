use crate::language::{
    pipeline::concepts::tokens::{token, Token, TokenIter},
    syntax::{Syntax, SyntaxElement, SyntaxFragment, SyntaxHandle},
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

    let syntax_element = match next_token {
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
    let token = expect::<token::Word>(tokens)?;
    Ok(token.0)
}

fn parse_number(tokens: &mut TokenIter) -> ParserResult<i64> {
    let token = expect::<token::Number>(tokens)?;
    Ok(token.0)
}

fn parse_symbol(tokens: &mut TokenIter) -> ParserResult<String> {
    let token = expect::<token::Symbol>(tokens)?;
    Ok(token.0)
}

fn expect<T>(tokens: &mut TokenIter) -> ParserResult<T>
where
    T: TryFrom<Token, Error = Token>,
{
    match tokens.next().ok_or(NoMoreTokens)?.try_into() {
        Ok(token) => Ok(token),
        Err(token) => Err(ParserError::UnexpectedToken { actual: token }),
    }
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
