use crate::language::repr::{
    eval::value::{self, Value},
    syntax::{
        Fragment, FragmentAddress, FragmentId, FragmentPayload, Fragments,
    },
    tokens::{token, Token},
};

use super::b_parser::{
    expect, parse_number, parse_symbol, parse_word, NoMoreTokens, ParserError,
    ParserResult, Tokens,
};

pub fn analyze(
    tokens: Vec<Token>,
    syntax: &mut Fragments,
) -> ParserResult<AnalyzerOutput> {
    let mut tokens = tokens.into_iter().peekable();
    let start = parse_fragment(None, &mut tokens, syntax)?;
    Ok(AnalyzerOutput { start })
}

fn parse_fragment(
    terminator: Option<Token>,
    tokens: &mut Tokens,
    syntax: &mut Fragments,
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
            FragmentPayload::Value(block.into())
        }
        Token::Word(_) => {
            let word = parse_word(tokens)?;
            FragmentPayload::Word(word)
        }
        Token::Number(_) => {
            let number = parse_number(tokens)?;
            FragmentPayload::Value(Value::Number(number))
        }
        Token::Symbol(_) => {
            let symbol = parse_symbol(tokens)?;
            FragmentPayload::Value(value::Symbol(symbol).into())
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
        syntax.add(Fragment::new(FragmentAddress { next }, payload));

    Ok(Some(fragment_id))
}

fn parse_block(
    tokens: &mut Tokens,
    syntax: &mut Fragments,
) -> ParserResult<Option<FragmentId>> {
    expect::<token::CurlyBracketOpen>(tokens)?;

    let fragment_id =
        parse_fragment(Some(Token::CurlyBracketClose), tokens, syntax)?;

    Ok(fragment_id)
}

pub struct AnalyzerOutput {
    pub start: Option<FragmentId>,
}
