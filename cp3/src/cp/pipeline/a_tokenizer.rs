use std::mem;

use crate::cp::tokens::{Token, Tokens};

pub struct Tokenizer {
    state: State,
}

#[derive(Debug)]
enum State {
    Searching,
    ProcessingAny { buf: String },
    ProcessingString { buf: String },
}

pub fn new() -> Tokenizer {
    Tokenizer {
        state: State::Searching,
    }
}

pub fn tokenize(code: impl IntoIterator<Item = char>) -> Tokens {
    let code = code.into_iter();
    let mut tokens = Vec::new();

    let mut tokenizer = new();

    for ch in code {
        push_char(ch, &mut tokenizer, &mut tokens);
    }

    if let State::ProcessingAny { buf } = tokenizer.state {
        Token::match_delimited(&buf, &mut tokens);
    }

    Tokens(tokens.into())
}

pub fn push_char(ch: char, tokenizer: &mut Tokenizer, tokens: &mut Vec<Token>) {
    match &mut tokenizer.state {
        State::Searching => {
            if ch.is_whitespace() {
                return;
            }

            if ch == '"' {
                tokenizer.state =
                    State::ProcessingString { buf: String::new() };
                return;
            }

            let buf = String::from(ch);
            tokenizer.state = State::ProcessingAny { buf }
        }
        State::ProcessingAny { buf } => {
            if Token::match_eagerly(buf, tokens) {
                buf.clear();

                if ch.is_whitespace() {
                    tokenizer.state = State::Searching;
                    return;
                }
            }

            if ch == '"' {
                Token::match_delimited(buf.as_str(), tokens);
                tokenizer.state =
                    State::ProcessingString { buf: String::new() };
                return;
            }

            if !ch.is_whitespace() {
                buf.push(ch);
                return;
            }

            if !buf.is_empty() {
                Token::match_delimited(buf.as_str(), tokens);
                tokenizer.state = State::Searching;
            }
        }
        State::ProcessingString { buf } => {
            if ch == '"' {
                let string = mem::take(buf);
                tokens.push(Token::String(string));
                tokenizer.state = State::Searching;
                return;
            }

            buf.push(ch);
        }
    }
}
