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
                    if let Some(update) = process_special_char(ch) {
                        match update {
                            SpecialCharUpdate::Token(token) => {
                                tokens.push(token)
                            }
                            SpecialCharUpdate::State(s) => state = s,
                        }
                    }
                }
                ch => {
                    state = State::Other {
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
                    state = finalize_symbol(buf, &mut tokens);
                }
                ch if is_special_char(ch) => match process_special_char(ch) {
                    Some(SpecialCharUpdate::Token(token)) => {
                        state = finalize_symbol(buf, &mut tokens);
                        tokens.push(token);
                    }
                    Some(SpecialCharUpdate::State(s)) => state = s,
                    None => state = State::Symbol { buf },
                },
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
            State::Other { mut buf } => match ch {
                ch if ch.is_whitespace() => {
                    state = finalize_other(buf, &mut tokens);
                }
                ch if is_special_char(ch) => match process_special_char(ch) {
                    Some(SpecialCharUpdate::Token(token)) => {
                        state = finalize_other(buf, &mut tokens);
                        tokens.push(token);
                    }
                    Some(SpecialCharUpdate::State(s)) => {
                        finalize_other(buf, &mut tokens);
                        state = s
                    }
                    None => state = State::Other { buf },
                },
                ch => {
                    buf.push(ch);
                    state = State::Other { buf };
                }
            },
        }
    }

    tokens
}

#[derive(Debug)]
enum State {
    Scanning,
    Comment,
    Symbol { buf: String },
    Text { buf: String },
    Other { buf: String },
}

// That whole deal with special characters here is not great:
//
// - There's too much duplication. It would be nicer, if the following code
//   would only need to specify a special character once.
// - I'd like to give the `=>` token the same treatment, but those are two
//   characters, which won't work with the current approach.

fn is_special_char(ch: char) -> bool {
    matches!(ch, '{' | '}' | '[' | ']' | '#' | ':' | '"')
}

fn process_special_char(ch: char) -> Option<SpecialCharUpdate> {
    match ch {
        '{' => Some(SpecialCharUpdate::Token(Token::CurlyBracketOpen)),
        '}' => Some(SpecialCharUpdate::Token(Token::CurlyBracketClose)),
        '[' => Some(SpecialCharUpdate::Token(Token::SquareBracketOpen)),
        ']' => Some(SpecialCharUpdate::Token(Token::SquareBracketClose)),
        '#' => Some(SpecialCharUpdate::State(State::Comment)),
        ':' => Some(SpecialCharUpdate::State(State::Symbol {
            buf: String::new(),
        })),
        '"' => {
            Some(SpecialCharUpdate::State(State::Text { buf: String::new() }))
        }
        _ => None,
    }
}

enum SpecialCharUpdate {
    Token(Token),
    State(State),
}

fn finalize_symbol(buf: String, tokens: &mut Vec<Token>) -> State {
    tokens.push(Token::Literal(ValuePayload::Symbol(buf)));
    State::Scanning
}

fn finalize_other(buf: String, tokens: &mut Vec<Token>) -> State {
    let token = if buf == "=>" {
        Token::Binding
    } else {
        match buf.parse::<i64>() {
            Ok(number) => Token::Literal(ValuePayload::Number(number)),
            Err(_) => Token::Word(buf),
        }
    };

    tokens.push(token);

    State::Scanning
}
