use std::convert::Infallible;

use crate::cp::tokens::{Keyword, Literal, Token};

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
                let _ = chars.next()?;
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
    assert_eq!(*chars.next()?, '"');

    let mut buf = String::new();

    loop {
        match *chars.next()? {
            '"' => return Ok(Token::Literal(Literal::String(buf))),
            ch => buf.push(ch),
        }
    }
}

fn read_other(
    chars: &mut StageInputReader<char>,
) -> Result<Token, PipelineError<Infallible>> {
    let mut buf = String::new();

    while let Ok(&ch) = chars.next() {
        if ch.is_whitespace() {
            break;
        }

        buf.push(ch);

        for (s, token) in DELIMITERS {
            let s = *s;
            match buf.split_once(s) {
                Some(("", _)) => return Ok(token.clone()),
                Some(_) => break,
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

    match buf.as_str() {
        "fn" => return Ok(Token::Keyword(Keyword::Fn)),
        "mod" => return Ok(Token::Keyword(Keyword::Mod)),
        "test" => return Ok(Token::Keyword(Keyword::Test)),
        _ => {}
    }

    Ok(Token::Ident(buf))
}

const DELIMITERS: &[(&str, Token)] = &[
    ("=>", Token::BindingOperator),
    (".", Token::Period),
    ("{", Token::CurlyBracketOpen),
    ("}", Token::CurlyBracketClose),
    ("[", Token::SquareBracketOpen),
    ("]", Token::SquareBracketClose),
];
