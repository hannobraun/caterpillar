use crate::language::{
    syntax::{
        Syntax, SyntaxElement, SyntaxFragment, SyntaxHandle, TokenRange,
        TokensToSyntax,
    },
    tokens::{token, Token, TokenIter},
    value::{self, Value},
};

pub fn parse(
    mut tokens: TokenIter,
    syntax: &mut Syntax,
) -> ParserResult<ParserOutput> {
    let mut tokens_to_syntax = TokensToSyntax::new();
    let (start, _) =
        parse_fragment(None, &mut tokens, syntax, &mut tokens_to_syntax)?;

    Ok(ParserOutput {
        start,
        tokens_to_syntax,
    })
}

fn parse_fragment(
    terminator: Option<Token>,
    tokens: &mut TokenIter,
    syntax: &mut Syntax,
    tokens_to_syntax: &mut TokensToSyntax,
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

    let (syntax_element, token_range) = match next_token.token {
        Token::CurlyBracketOpen => {
            let (block, token_range) =
                parse_block(tokens, syntax, tokens_to_syntax)?;
            let block = SyntaxElement::Value(value::Block(block).into());
            (block, token_range)
        }
        Token::Word(_) => {
            let (word, token_range) = parse_word(tokens)?;
            let word = SyntaxElement::Word(word);
            (word, token_range)
        }
        Token::Number(_) => {
            let (number, token_range) = parse_number(tokens)?;
            let number = SyntaxElement::Value(Value::Number(number));
            (number, token_range)
        }
        Token::Symbol(_) => {
            let (symbol, token_range) = parse_symbol(tokens)?;
            let symbol = SyntaxElement::Value(value::Symbol(symbol).into());
            (symbol, token_range)
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

    let (next, hash) =
        parse_fragment(terminator, tokens, syntax, tokens_to_syntax)?;
    let handle = syntax.add(SyntaxFragment {
        payload: syntax_element,
        next,
    });
    tokens_to_syntax.insert(token_range, handle);

    Ok((Some(handle), hash))
}

fn parse_block(
    tokens: &mut TokenIter,
    syntax: &mut Syntax,
    tokens_to_syntax: &mut TokensToSyntax,
) -> ParserResult<(Option<SyntaxHandle>, TokenRange)> {
    let (_, start) = expect::<token::CurlyBracketOpen>(tokens)?;

    let (handle, end) = parse_fragment(
        Some(Token::CurlyBracketClose),
        tokens,
        syntax,
        tokens_to_syntax,
    )?;

    let end = end.unwrap();
    let range = TokenRange { start, end };

    Ok((handle, range))
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

pub struct ParserOutput {
    pub start: Option<SyntaxHandle>,
    pub tokens_to_syntax: TokensToSyntax,
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
