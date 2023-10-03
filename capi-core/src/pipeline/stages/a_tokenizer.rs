use std::iter;

use crate::repr::{eval::value::ValueKind, tokens::Token};

pub fn tokenize(code: &str) -> Vec<Token> {
    // Make sure that the code always ends on whitespace. Otherwise the
    // tokenizer might miss the last token.
    let mut chars = code.chars().chain(iter::once('\n'));

    let mut tokens = Vec::new();
    let mut state = State::Scanning;

    loop {
        let Some(ch) = chars.next() else { break };

        match state {
            State::Scanning => match ch {
                ch if ch.is_whitespace() => {
                    continue;
                }
                '{' => {
                    tokens.push(Token::CurlyBracketOpen);
                }
                '}' => {
                    tokens.push(Token::CurlyBracketClose);
                }
                '[' => {
                    tokens.push(Token::SquareBracketOpen);
                }
                '#' => {
                    state = State::Comment;
                }
                ':' => {
                    state = State::Symbol { buf: String::new() };
                }
                '"' => {
                    state = State::Text { buf: String::new() };
                }
                ch => {
                    state = State::WordOrNumber {
                        buf: String::from(ch),
                    };
                }
            },
            State::Comment => {
                if ch == '\n' {
                    state = State::Scanning;
                }
            }
            State::Symbol { mut buf } => {
                if ch.is_whitespace() {
                    tokens.push(Token::Literal(ValueKind::Symbol(buf)));
                    state = State::Scanning;
                    continue;
                }

                buf.push(ch);
                state = State::Symbol { buf };
            }
            State::Text { mut buf } => {
                if ch == '"' {
                    tokens.push(Token::Literal(ValueKind::Text(buf)));
                    state = State::Scanning;
                    continue;
                }

                buf.push(ch);
                state = State::Text { buf }
            }
            State::WordOrNumber { mut buf } => {
                if ch.is_whitespace() {
                    let token = match buf.parse::<i64>() {
                        Ok(number) => Token::Literal(ValueKind::Number(number)),
                        Err(_) => Token::Word(buf),
                    };

                    tokens.push(token);

                    state = State::Scanning;
                    continue;
                }

                buf.push(ch);
                state = State::WordOrNumber { buf };
            }
        }
    }

    tokens
}

enum State {
    Scanning,
    Comment,
    Symbol { buf: String },
    Text { buf: String },
    WordOrNumber { buf: String },
}
