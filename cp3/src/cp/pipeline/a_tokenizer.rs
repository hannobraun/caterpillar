use std::mem;

use crate::cp::tokens::{Token, Tokens};

pub fn tokenize(code: impl IntoIterator<Item = char>) -> Tokens {
    let code = code.into_iter();
    let mut tokens = Vec::new();

    let mut state = State::Searching;

    for ch in code {
        match &mut state {
            State::Searching => {
                if ch.is_whitespace() {
                    continue;
                }

                if ch == '"' {
                    state = State::ProcessingString { buf: String::new() };
                    continue;
                }

                let buf = String::from(ch);
                state = State::ProcessingAny { buf }
            }
            State::ProcessingAny { buf } => {
                if Token::match_eagerly(buf, &mut tokens) {
                    buf.clear();
                }

                if ch == '"' {
                    Token::match_delimited(buf.as_str(), &mut tokens);
                    state = State::ProcessingString { buf: String::new() };
                    continue;
                }

                if !ch.is_whitespace() {
                    buf.push(ch);
                    continue;
                }

                if !buf.is_empty() {
                    Token::match_delimited(buf.as_str(), &mut tokens);
                    state = State::Searching;
                }
            }
            State::ProcessingString { buf } => {
                if ch == '"' {
                    let string = mem::take(buf);
                    tokens.push(Token::String(string));
                    state = State::Searching;
                    continue;
                }

                buf.push(ch);
            }
        }
    }

    if let State::ProcessingAny { buf } = state {
        Token::match_delimited(&buf, &mut tokens);
    }

    Tokens(tokens.into())
}

enum State {
    Searching,
    ProcessingAny { buf: String },
    ProcessingString { buf: String },
}
