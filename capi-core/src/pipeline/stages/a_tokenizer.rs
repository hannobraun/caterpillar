use std::iter;

use crate::repr::{eval::value::ValuePayload, tokens::Token};

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
                    // Whitespace is ignored in this state.
                }
                ch if is_special_char(ch) => {
                    let (s, t) = process_special_char(ch);

                    if let Some(s) = s {
                        state = s;
                    }

                    tokens.extend(t);
                }
                ch => {
                    state = State::WordOrNumber {
                        buf: String::from(ch),
                    };
                }
            },
            State::Comment => {
                // Comments are ended by a newline. Anything else doesn't matter
                // here.
                if ch == '\n' {
                    state = State::Scanning;
                }
            }
            State::Symbol { mut buf } => match ch {
                ch if ch.is_whitespace() => {
                    tokens.push(Token::Literal(ValuePayload::Symbol(buf)));
                    state = State::Scanning;
                }
                ch if is_special_char(ch) => {
                    let (s, t) = process_special_char(ch);

                    match s {
                        Some(s) => {
                            state = s;
                        }
                        None => {
                            state = State::Symbol { buf };
                        }
                    }

                    tokens.extend(t);
                }
                ch => {
                    buf.push(ch);
                    state = State::Symbol { buf };
                }
            },
            State::Text { mut buf } => match ch {
                '"' => {
                    tokens.push(Token::Literal(ValuePayload::Text(buf)));
                    state = State::Scanning;
                }
                ch => {
                    buf.push(ch);
                    state = State::Text { buf }
                }
            },
            State::WordOrNumber { mut buf } => match ch {
                ch if ch.is_whitespace() => {
                    let token = match buf.parse::<i64>() {
                        Ok(number) => {
                            Token::Literal(ValuePayload::Number(number))
                        }
                        Err(_) => Token::Word(buf),
                    };

                    tokens.push(token);

                    state = State::Scanning;
                }
                ch => {
                    buf.push(ch);
                    state = State::WordOrNumber { buf };
                }
            },
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

fn is_special_char(ch: char) -> bool {
    matches!(ch, '{' | '}' | '[' | ']' | '#' | ':' | '"')
}

fn process_special_char(ch: char) -> (Option<State>, Option<Token>) {
    match ch {
        '{' => (None, Some(Token::CurlyBracketOpen)),
        '}' => (None, Some(Token::CurlyBracketClose)),
        '[' => (None, Some(Token::SquareBracketOpen)),
        ']' => (None, Some(Token::SquareBracketClose)),
        '#' => (Some(State::Comment), None),
        ':' => (Some(State::Symbol { buf: String::new() }), None),
        '"' => (Some(State::Text { buf: String::new() }), None),
        _ => (None, None),
    }
}
