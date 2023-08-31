use crate::language::repr::{
    eval::value::{self, Value},
    syntax::{
        FragmentAddress, FragmentId, Syntax, SyntaxElement, SyntaxFragment,
    },
    tokens::{token, Token},
};

use super::b_parser::{NoMoreTokens, ParserError, ParserResult, Tokens};

pub fn analyze(
    tokens: Vec<Token>,
    syntax: &mut Syntax,
) -> ParserResult<AnalyzerOutput> {
    let mut tokens = tokens.into_iter().peekable();
    let start = parse_fragment(None, &mut tokens, syntax)?;
    Ok(AnalyzerOutput { start })
}

fn parse_fragment(
    terminator: Option<Token>,
    tokens: &mut Tokens,
    syntax: &mut Syntax,
) -> ParserResult<Option<FragmentId>> {
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
        _ => {
            // Only peeked before; still need to consume.
            let token = tokens.next().unwrap();

            if Some(&token) == terminator.as_ref() {
                return Ok(None);
            }

            return Err(ParserError::UnexpectedToken { actual: token });
        }
    };

    let next = parse_fragment(terminator, tokens, syntax)?;
    let fragment_id =
        syntax.add(SyntaxFragment::new(payload, FragmentAddress { next }));

    Ok(Some(fragment_id))
}

fn parse_block(
    tokens: &mut Tokens,
    syntax: &mut Syntax,
) -> ParserResult<Option<FragmentId>> {
    expect::<token::CurlyBracketOpen>(tokens)?;

    let fragment_id =
        parse_fragment(Some(Token::CurlyBracketClose), tokens, syntax)?;

    Ok(fragment_id)
}

fn parse_word(tokens: &mut Tokens) -> ParserResult<String> {
    let payload = expect::<token::Word>(tokens)?;
    Ok(payload.0)
}

fn parse_number(tokens: &mut Tokens) -> ParserResult<i64> {
    let payload = expect::<token::Number>(tokens)?;
    Ok(payload.0)
}

fn parse_symbol(tokens: &mut Tokens) -> ParserResult<String> {
    let payload = expect::<token::Symbol>(tokens)?;
    Ok(payload.0)
}

fn expect<T>(tokens: &mut Tokens) -> ParserResult<T>
where
    T: TryFrom<Token, Error = Token>,
{
    let token = tokens.next().ok_or(NoMoreTokens)?;

    let payload = token
        .try_into()
        .map_err(|token| ParserError::UnexpectedToken { actual: token })?;

    Ok(payload)
}

pub struct AnalyzerOutput {
    pub start: Option<FragmentId>,
}
