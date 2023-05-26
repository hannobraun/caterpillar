use std::{collections::VecDeque, convert::Infallible};

use super::{stage_input::NoMoreInput, PipelineError};

pub fn tokenize(
    chars: &mut VecDeque<char>,
) -> Result<Token, PipelineError<Infallible>> {
    let mut buf = String::new();

    while let Some(ch) = chars.pop_front() {
        if ch.is_whitespace() && buf.is_empty() {
            continue;
        }
        if ch.is_whitespace() {
            break;
        }

        buf.push(ch);

        match buf.as_str() {
            "{" => return Ok(Token::CurlyBracketOpen),
            "}" => return Ok(Token::CurlyBracketClose),
            "fn" => return Ok(Token::Fn),
            "mod" => return Ok(Token::Mod),
            _ => {}
        }
    }

    if buf.is_empty() {
        return Err(PipelineError::NotEnoughInput(NoMoreInput));
    }

    Ok(Token::Ident(buf))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    CurlyBracketOpen,
    CurlyBracketClose,
    Fn,
    Mod,
    Ident(String),
}
