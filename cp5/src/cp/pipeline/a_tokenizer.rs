use std::convert::Infallible;

use crate::cp::tokens::{Literal, Token, DELIMITERS, KEYWORDS};

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
    loop {
        match *chars.peek()? {
            '"' => return read_string(chars),
            ch if ch.is_whitespace() => {
                let _ = chars.read()?;
                chars.take();
                continue;
            }
            _ => return read_other(chars),
        }
    }
}

fn read_string(
    chars: &mut StageInputReader<char>,
) -> Result<Token, PipelineError<Infallible>> {
    // This method is only ever called, if this is true. If it isn't, that's a
    // bug in this module.
    assert_eq!(*chars.read()?, '"');

    let mut buf = String::new();

    loop {
        match *chars.read()? {
            '"' => return Ok(Token::Literal(Literal::String(buf))),
            ch => buf.push(ch),
        }
    }
}

fn read_other(
    chars: &mut StageInputReader<char>,
) -> Result<Token, PipelineError<Infallible>> {
    let mut buf = String::new();

    while let Ok(&ch) = chars.read() {
        if ch.is_whitespace() {
            break;
        }

        buf.push(ch);

        for (s, token) in DELIMITERS {
            match buf.split_once(*s) {
                Some(("", _)) => return Ok(token.clone()),
                Some((buf, _)) => {
                    chars.unread_last_n(s.chars().count());
                    return read_keyword_or_ident(buf.into());
                }
                None => {}
            };
        }
    }

    read_keyword_or_ident(buf)
}

fn read_keyword_or_ident(
    buf: String,
) -> Result<Token, PipelineError<Infallible>> {
    if buf.is_empty() {
        return Err(PipelineError::NotEnoughInput(NoMoreInput));
    }

    for (s, token) in KEYWORDS {
        if buf == *s {
            return Ok(token.clone());
        }
    }

    Ok(Token::Ident(buf))
}
