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
        push_char(ch, &mut tokenizer, &mut tokens);
    }
    finalize(tokenizer, &mut tokens);

    Tokens(tokens.into())
}

pub fn push_char(ch: char, tokenizer: &mut Tokenizer, tokens: &mut Vec<Token>) {
    match tokenizer {
        Tokenizer::Searching => {
            if ch.is_whitespace() {
                return;
            }

            if ch == '"' {
                *tokenizer = Tokenizer::ProcessingString { buf: String::new() };
                return;
            }

            let buf = String::from(ch);
            *tokenizer = Tokenizer::ProcessingAny { buf }
        }
        Tokenizer::ProcessingAny { buf } => {
            let t = Token::match_eagerly(buf);
            if !t.is_empty() {
                tokens.extend(t);
                buf.clear();

                if ch.is_whitespace() {
                    *tokenizer = Tokenizer::Searching;
                    return;
                }
            }

            if ch == '"' {
                tokens.extend(Token::match_delimited(buf.as_str()));
                *tokenizer = Tokenizer::ProcessingString { buf: String::new() };
                return;
            }

            if !ch.is_whitespace() {
                buf.push(ch);
                return;
            }

            if !buf.is_empty() {
                tokens.extend(Token::match_delimited(buf.as_str()));
                *tokenizer = Tokenizer::Searching;
            }
        }
        Tokenizer::ProcessingString { buf } => {
            if ch == '"' {
                let string = mem::take(buf);
                tokens.push(Token::String(string));
                *tokenizer = Tokenizer::Searching;
                return;
            }

            buf.push(ch);
        }
    }
}

pub fn finalize(tokenizer: Tokenizer, tokens: &mut Vec<Token>) {
    if let Tokenizer::ProcessingAny { buf } = tokenizer {
        tokens.extend(Token::match_delimited(&buf));
    }
}
