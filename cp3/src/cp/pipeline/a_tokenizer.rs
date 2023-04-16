use std::{iter, option};

use map_macro::map;

use crate::cp::{
    keywords::Keyword,
    tokens::{Token, STRING_DELIMITER},
};

#[derive(Debug)]
pub struct Tokenizer {
    buf: Buf,
    state: State,
}

#[derive(Debug)]
struct Buf(String);

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
        buf: Buf(String::new()),
        state: State::Searching,
    }
}

pub fn push_char(ch: char, mut tokenizer: Tokenizer) -> (Tokenizer, Tokens) {
    let (next_state, tokens) = match ch {
        STRING_DELIMITER => match tokenizer.state {
            State::Searching => {
                tokenizer.buf.0.clear();
                (State::ProcessingString, Tokens::Zero)
            }
            State::ProcessingAny => {
                let tokens = match match_delimited(&tokenizer.buf.0) {
                    Some(token) => Tokens::One(token),
                    None => Tokens::Zero,
                };

                tokenizer.buf.0.clear();
                (State::ProcessingString, tokens)
            }
            State::ProcessingString => {
                let token = Token::String(tokenizer.buf.0.clone());

                tokenizer.buf.0.clear();
                (State::Searching, Tokens::One(token))
            }
        },
        ch if ch.is_whitespace() => match tokenizer.state {
            State::Searching => (State::Searching, Tokens::Zero),
            State::ProcessingAny => match match_delimited(&tokenizer.buf.0) {
                Some(token) => {
                    tokenizer.buf.0.clear();
                    (State::Searching, Tokens::One(token))
                }
                None => (State::ProcessingAny, Tokens::Zero),
            },
            State::ProcessingString => {
                tokenizer.buf.0.push(ch);
                (State::ProcessingString, Tokens::Zero)
            }
        },
        ch => {
            tokenizer.buf.0.push(ch);

            match tokenizer.state {
                State::Searching | State::ProcessingAny => {
                    let tokens = match_eagerly(&tokenizer.buf.0);
                    if tokens != Tokens::Zero {
                        tokenizer.buf.0.clear();
                    }

                    (State::ProcessingAny, tokens)
                }
                State::ProcessingString => {
                    (State::ProcessingString, Tokens::Zero)
                }
            }
        }
    };

    tokenizer.state = next_state;
    (tokenizer, tokens)
}

pub fn finalize(tokenizer: Tokenizer) -> Vec<Token> {
    let mut tokens = Vec::new();

    if let State::ProcessingAny = tokenizer.state {
        tokens.extend(match_eagerly(&tokenizer.buf.0));
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
