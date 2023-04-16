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
    let (next_state, tokens) = match ch {
        STRING_DELIMITER => match tokenizer.state {
            State::Searching => {
                tokenizer.buf.clear();
                (State::ProcessingString, vec![])
            }
            State::ProcessingAny => {
                let mut tokens = Vec::new();
                tokens.extend(match_delimited(&tokenizer.buf));

                tokenizer.buf.clear();
                (State::ProcessingString, tokens)
            }
            State::ProcessingString => {
                let string = mem::take(&mut tokenizer.buf);
                let token = Token::String(string);

                (State::Searching, vec![token])
            }
        },
        ch if ch.is_whitespace() => match tokenizer.state {
            State::Searching => (State::Searching, vec![]),
            State::ProcessingAny => {
                let t = match_delimited(&tokenizer.buf);

                let next_state = match &t {
                    Some(_) => State::Searching,
                    None => State::ProcessingAny,
                };

                let mut tokens = Vec::new();
                tokens.extend(t);

                (next_state, tokens)
            }
            State::ProcessingString => {
                tokenizer.buf.push(ch);
                (State::ProcessingString, vec![])
            }
        },
        ch => {
            if let State::Searching = tokenizer.state {
                tokenizer.buf.clear();
            }

            tokenizer.buf.push(ch);

            match tokenizer.state {
                State::Searching | State::ProcessingAny => {
                    let mut tokens = Vec::new();

                    let t = match_eagerly(&tokenizer.buf);
                    tokens.extend(t);
                    if !tokens.is_empty() {
                        tokenizer.buf.clear();
                    }

                    (State::ProcessingAny, tokens)
                }
                State::ProcessingString => (State::ProcessingString, vec![]),
            }
        }
    };

    tokenizer.state = next_state;
    (tokenizer, tokens)
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
