use std::{iter, mem, option};

use map_macro::map;

use crate::cp::{
    keywords::Keyword,
    tokens::{Token, STRING_DELIMITER},
};

#[derive(Debug)]
pub struct Tokenizer {
    buf: String,
    state: State,
}

#[derive(Debug)]
enum State {
    Searching,
    ProcessingAny,
    ProcessingString,
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
        buf: String::new(),
        state: State::Searching,
    }
}

pub fn push_char(
    ch: char,
    mut tokenizer: Tokenizer,
) -> (Tokenizer, Vec<Token>) {
    let next_state = match ch {
        STRING_DELIMITER => match tokenizer.state {
            State::Searching => {
                return (
                    Tokenizer {
                        buf: String::new(),
                        state: State::ProcessingString,
                    },
                    vec![],
                );
            }
            State::ProcessingAny => {
                let mut tokens = Vec::new();
                tokens.extend(match_delimited(&tokenizer.buf));

                return (
                    Tokenizer {
                        buf: String::new(),
                        state: State::ProcessingString,
                    },
                    tokens,
                );
            }
            State::ProcessingString => {
                let string = mem::take(&mut tokenizer.buf);
                let token = Token::String(string);

                return (
                    Tokenizer {
                        buf: tokenizer.buf,
                        state: State::Searching,
                    },
                    vec![token],
                );
            }
        },
        ch if ch.is_whitespace() => match tokenizer.state {
            State::Searching => return (tokenizer, vec![]),
            State::ProcessingAny => {
                let t = match_delimited(&tokenizer.buf);

                let next_state = match &t {
                    Some(_) => State::Searching,
                    None => State::ProcessingAny,
                };

                let mut tokens = Vec::new();
                tokens.extend(t);

                return (
                    Tokenizer {
                        buf: tokenizer.buf,
                        state: next_state,
                    },
                    tokens,
                );
            }
            State::ProcessingString => {
                tokenizer.buf.push(ch);

                return (
                    Tokenizer {
                        buf: tokenizer.buf,
                        state: State::ProcessingString,
                    },
                    vec![],
                );
            }
        },
        ch => {
            if let State::Searching = tokenizer.state {
                tokenizer.buf.clear();
            }

            match tokenizer.state {
                State::Searching | State::ProcessingAny => {
                    tokenizer.buf.push(ch);
                    State::ProcessingAny
                }
                State::ProcessingString => {
                    tokenizer.buf.push(ch);

                    State::ProcessingString
                }
            }
        }
    };

    match next_state {
        State::ProcessingAny => {
            let mut tokens = Vec::new();

            let t = match_eagerly(&tokenizer.buf);
            if t != Tokens::Zero {
                tokens.extend(t);
                tokenizer.buf.clear();

                if ch.is_whitespace() {
                    return (
                        Tokenizer {
                            buf: tokenizer.buf,
                            state: State::Searching,
                        },
                        tokens,
                    );
                }
            }

            (
                Tokenizer {
                    buf: tokenizer.buf.clone(),
                    state: State::ProcessingAny,
                },
                tokens,
            )
        }
        state => (
            Tokenizer {
                buf: tokenizer.buf,
                state,
            },
            vec![],
        ),
    }
}

pub fn finalize(tokenizer: Tokenizer) -> Vec<Token> {
    let mut tokens = Vec::new();

    if let State::ProcessingAny = tokenizer.state {
        tokens.extend(match_eagerly(&tokenizer.buf));
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
