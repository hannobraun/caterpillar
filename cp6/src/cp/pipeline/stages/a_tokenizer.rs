use std::convert::Infallible;

use crate::cp::{
    pipeline::{
        channel::{NoMoreInput, StageInput},
        ir::tokens::{Literal, Token, DELIMITERS, KEYWORDS, STRING_DELIMITER},
    },
    PipelineError,
};

pub fn tokenize(mut chars: StageInput<char>) -> Result<Token> {
    let token = tokenize_inner(&mut chars)?;
    chars.take();
    Ok(token)
}

fn tokenize_inner(chars: &mut StageInput<char>) -> Result<Token> {
    loop {
        match *chars.peek()? {
            STRING_DELIMITER => return read_string(chars),
            ch if ch.is_whitespace() => {
                let _ = chars.read()?;
                chars.take();
                continue;
            }
            _ => return read_other(chars),
        }
    }
}

fn read_string(chars: &mut StageInput<char>) -> Result<Token> {
    // This method is only ever called, if this is true. If it isn't, that's a
    // bug in this module.
    assert_eq!(*chars.read()?, STRING_DELIMITER);

    let mut buf = String::new();

    loop {
        match *chars.read()? {
            STRING_DELIMITER => {
                return Ok(Token::Literal(Literal::String(buf)))
            }
            ch => buf.push(ch),
        }
    }
}

fn read_other(chars: &mut StageInput<char>) -> Result<Token> {
    let mut buf = String::new();

    while let Ok(&ch) = chars.peek() {
        if ch.is_whitespace() || ch == STRING_DELIMITER {
            break;
        }

        buf.push(ch);
        let _ = chars.read();

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

fn read_keyword_or_ident(buf: String) -> Result<Token> {
    if buf.is_empty() {
        return Err(PipelineError::NotEnoughInput(NoMoreInput));
    }

    if let Ok(number) = buf.parse::<u8>() {
        return Ok(Token::Literal(Literal::Number(number)));
    }

    for (s, token) in KEYWORDS {
        if buf == *s {
            return Ok(token.clone());
        }
    }

    Ok(Token::Ident(buf))
}

type Result<T> = std::result::Result<T, PipelineError<Infallible>>;
