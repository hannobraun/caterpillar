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
                    let s = process_special_char(ch, &mut tokens);

                    if let Some(s) = s {
                        state = s;
                    }
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
                    match process_special_char(ch, &mut tokens) {
                        Some(s) => {
                            state = s;
                        }
                        None => {
                            state = State::Symbol { buf };
                        }
                    }
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

fn is_special_char(ch: char) -> bool {
    matches!(ch, '{' | '}' | '[' | ']' | '#' | ':' | '"')
}

fn process_special_char(ch: char, tokens: &mut Vec<Token>) -> Option<State> {
    match ch {
        '{' => {
            tokens.push(Token::CurlyBracketOpen);
        }
        '}' => {
            tokens.push(Token::CurlyBracketClose);
        }
        '[' => {
            tokens.push(Token::SquareBracketOpen);
        }
        ']' => {
            tokens.push(Token::SquareBracketClose);
        }
        '#' => {
            return Some(State::Comment);
        }
        ':' => {
            return Some(State::Symbol { buf: String::new() });
        }
        '"' => {
            return Some(State::Text { buf: String::new() });
        }
        _ => {}
    }

    None
}

enum State {
    Scanning,
    Comment,
    Symbol { buf: String },
    Text { buf: String },
    WordOrNumber { buf: String },
}
