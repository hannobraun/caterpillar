use std::convert::Infallible;

use super::{
    stage_input::{NoMoreInput, StageInputReader},
    PipelineError,
};

pub fn tokenize(
    mut chars: StageInputReader<char>,
) -> Result<Token, PipelineError<Infallible>> {
    let token = tokenize_inner(&mut chars)?;
    chars.take();
    Ok(token)
}

fn tokenize_inner(
    chars: &mut StageInputReader<char>,
) -> Result<Token, PipelineError<Infallible>> {
    let mut buf = String::new();

    while let Ok(&ch) = chars.next() {
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
            _ => {}
        }
    }

    if buf.is_empty() {
        return Err(PipelineError::NotEnoughInput(NoMoreInput));
    }

    match buf.as_str() {
        "fn" => return Ok(Token::Fn),
        "mod" => return Ok(Token::Mod),
        "test" => return Ok(Token::Test),
        _ => {}
    }

    Ok(Token::Ident(buf))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    CurlyBracketOpen,
    CurlyBracketClose,
    Fn,
    Mod,
    Test,
    Ident(String),
}
