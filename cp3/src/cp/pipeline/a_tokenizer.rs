use std::{iter, mem, option};

use map_macro::map;

use crate::cp::{
    keywords::Keyword,
    tokens::{Token, STRING_DELIMITER},
};

#[derive(Debug)]
pub struct Tokenizer {
    state: State,
}

#[derive(Debug)]
enum State {
    Searching,
    ProcessingAny { buf: String },
    ProcessingString { buf: String },
}

#[derive(Eq, PartialEq)]
pub enum Tokens {
    Zero,
    One(Token),
    Two(Token, Token),
}

impl IntoIterator for Tokens {
    type Item = Token;
    type IntoIter =
        iter::Chain<option::IntoIter<Token>, option::IntoIter<Token>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Tokens::Zero => None.into_iter().chain(None),
            Tokens::One(token) => Some(token).into_iter().chain(None),
            Tokens::Two(a, b) => Some(a).into_iter().chain(Some(b)),
        }
    }
}

pub fn tokenizer() -> Tokenizer {
    Tokenizer {
        state: State::Searching,
    }
}

pub fn push_char(ch: char, tokenizer: Tokenizer) -> (Tokenizer, Vec<Token>) {
    let tokenizer = match tokenizer.state {
        State::Searching => {
            if ch.is_whitespace() {
                return (tokenizer, vec![]);
            }

            if ch == STRING_DELIMITER {
                return (
                    Tokenizer {
                        state: State::ProcessingString { buf: String::new() },
                    },
                    vec![],
                );
            }

            let buf = String::from(ch);
            State::ProcessingAny { buf }
        }
        State::ProcessingAny { mut buf } => {
            if ch == STRING_DELIMITER {
                let mut tokens = Vec::new();
                tokens.extend(match_delimited(&buf));

                return (
                    Tokenizer {
                        state: State::ProcessingString { buf: String::new() },
                    },
                    tokens,
                );
            }

            if ch.is_whitespace() {
                let t = match_delimited(&buf);

                let next_state = match &t {
                    Some(_) => State::Searching,
                    None => State::ProcessingAny { buf },
                };

                let mut tokens = Vec::new();
                tokens.extend(t);

                return (Tokenizer { state: next_state }, tokens);
            }

            buf.push(ch);
            State::ProcessingAny { buf }
        }
        State::ProcessingString { mut buf } => {
            if ch == STRING_DELIMITER {
                let string = mem::take(&mut buf);
                let token = Token::String(string);

                return (
                    Tokenizer {
                        state: State::Searching,
                    },
                    vec![token],
                );
            }

            buf.push(ch);

            State::ProcessingString { buf }
        }
    };

    match tokenizer {
        State::ProcessingAny { mut buf } => {
            let mut tokens = Vec::new();

            let t = match_eagerly(&buf);
            if t != Tokens::Zero {
                tokens.extend(t);
                buf.clear();

                if ch.is_whitespace() {
                    return (
                        Tokenizer {
                            state: State::Searching,
                        },
                        tokens,
                    );
                }
            }

            (
                Tokenizer {
                    state: State::ProcessingAny { buf },
                },
                tokens,
            )
        }
        state => (Tokenizer { state }, vec![]),
    }
}

pub fn finalize(tokenizer: Tokenizer) -> Vec<Token> {
    let mut tokens = Vec::new();

    if let State::ProcessingAny { buf } = tokenizer.state {
        tokens.extend(match_eagerly(&buf));
    }

    tokens
}

fn match_eagerly(buf: &str) -> Tokens {
    let delimiters = map! {
        "=>" => Token::BindingOperator,
        "." => Token::Period,
        "{" => Token::CurlyBracketOpen,
        "}" => Token::CurlyBracketClose,
        "(" => Token::RoundBracketOpen,
        ")" => Token::RoundBracketClose,
        "[" => Token::SquareBracketOpen,
        "]" => Token::SquareBracketClose,
    };

    for (delimiter, token) in delimiters {
        if let Some((first_token, "")) = buf.split_once(delimiter) {
            return match match_delimited(first_token) {
                Some(first_token) => Tokens::Two(first_token, token),
                None => Tokens::One(token),
            };
        }
    }

    Tokens::Zero
}

fn match_delimited(buf: &str) -> Option<Token> {
    // This function is only ever called with a `buf` that has already been
    // matched against delimiters before. We don't need to check again, whether
    // any delimiters are in here.

    if let Some(keyword) = Keyword::parse(buf) {
        return Some(Token::Keyword(keyword));
    }
    if let Some(("", symbol)) = buf.split_once(':') {
        return Some(Token::Symbol(symbol.into()));
    }

    if buf.is_empty() {
        None
    } else {
        Some(Token::Ident(buf.into()))
    }
}
