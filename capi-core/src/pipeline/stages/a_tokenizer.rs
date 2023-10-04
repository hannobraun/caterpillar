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
                    // Whitespace is ignore in this state.
                }
                ch if process_char_eagerly(ch, &mut tokens, &mut state) => {
                    // That call to `process_eager_token` already performs all
                    // the work that's needed for this case. Nothing else left
                    // to do here!
                }
                ch => {
                    state = State::WordOrNumber {
                        buf: String::from(ch),
                    };
                }
            },
            State::Comment => {
                // Comments are ended by a newline. Anything else doesn't
                // matter here.
                if ch == '\n' {
                    state = State::Scanning;
                }
            }
            State::Symbol { mut buf } => {
                if ch.is_whitespace() {
                    tokens.push(Token::Literal(ValuePayload::Symbol(buf)));
                    state = State::Scanning;
                    continue;
                }

                buf.push(ch);
                state = State::Symbol { buf };
            }
            State::Text { mut buf } => {
                if ch == '"' {
                    tokens.push(Token::Literal(ValuePayload::Text(buf)));
                    state = State::Scanning;
                    continue;
                }

                buf.push(ch);
                state = State::Text { buf }
            }
            State::WordOrNumber { mut buf } => {
                if ch.is_whitespace() {
                    let token = match buf.parse::<i64>() {
                        Ok(number) => {
                            Token::Literal(ValuePayload::Number(number))
                        }
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

fn process_char_eagerly(
    ch: char,
    tokens: &mut Vec<Token>,
    state: &mut State,
) -> bool {
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
            *state = State::Comment;
        }
        ':' => {
            *state = State::Symbol { buf: String::new() };
        }
        '"' => {
            *state = State::Text { buf: String::new() };
        }
        _ => {
            return false;
        }
    }

    true
}

enum State {
    Scanning,
    Comment,
    Symbol { buf: String },
    Text { buf: String },
    WordOrNumber { buf: String },
}
