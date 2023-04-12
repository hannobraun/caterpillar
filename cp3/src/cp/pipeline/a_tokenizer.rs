use std::mem;

use crate::cp::tokens::{Token, Tokens};

#[derive(Debug)]
pub enum Tokenizer {
    Searching,
    ProcessingAny { buf: String },
    ProcessingString { buf: String },
}

pub fn new() -> Tokenizer {
    Tokenizer::Searching
}

pub fn tokenize(code: impl IntoIterator<Item = char>) -> Tokens {
    let code = code.into_iter();
    let mut tokens = Vec::new();

    let mut tokenizer = new();

    for ch in code {
        tokenizer = push_char(ch, tokenizer, &mut tokens);
    }
    tokens.extend(finalize(tokenizer));

    Tokens(tokens.into())
}

pub fn push_char(
    ch: char,
    tokenizer: Tokenizer,
    tokens: &mut Vec<Token>,
) -> Tokenizer {
    match tokenizer {
        Tokenizer::Searching => {
            if ch.is_whitespace() {
                return tokenizer;
            }

            if ch == '"' {
                return Tokenizer::ProcessingString { buf: String::new() };
            }

            let buf = String::from(ch);
            Tokenizer::ProcessingAny { buf }
        }
        Tokenizer::ProcessingAny { mut buf } => {
            let t = Token::match_eagerly(&buf);
            if !t.is_empty() {
                tokens.extend(t);
                buf.clear();

                if ch.is_whitespace() {
                    return Tokenizer::Searching;
                }
            }

            if ch == '"' {
                tokens.extend(Token::match_delimited(buf.as_str()));
                return Tokenizer::ProcessingString { buf: String::new() };
            }

            if !ch.is_whitespace() {
                buf.push(ch);
                return Tokenizer::ProcessingAny { buf };
            }

            if !buf.is_empty() {
                tokens.extend(Token::match_delimited(buf.as_str()));
                return Tokenizer::Searching;
            }

            Tokenizer::ProcessingAny { buf }
        }
        Tokenizer::ProcessingString { mut buf } => {
            if ch == '"' {
                let string = mem::take(&mut buf);
                tokens.push(Token::String(string));
                return Tokenizer::Searching;
            }

            buf.push(ch);

            Tokenizer::ProcessingString { buf }
        }
    }
}

pub fn finalize(tokenizer: Tokenizer) -> Vec<Token> {
    if let Tokenizer::ProcessingAny { buf } = tokenizer {
        return Token::match_delimited(&buf);
    }

    vec![]
}
