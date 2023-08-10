use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

use enum_variant_type::EnumVariantType;

#[derive(Clone)]
pub struct Tokens {
    inner: VecDeque<Token>,
}

impl Tokens {
    pub fn peek(&self) -> Result<Token, NoMoreTokens> {
        self.inner.front().cloned().ok_or(NoMoreTokens)
    }

    pub fn next(&mut self) -> Result<Token, NoMoreTokens> {
        self.inner.pop_front().ok_or(NoMoreTokens)
    }
}

impl From<Vec<Token>> for Tokens {
    fn from(tokens: Vec<Token>) -> Self {
        Self {
            inner: tokens.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, EnumVariantType)]
#[evt(module = "token")]
pub enum Token {
    CurlyBracketOpen,
    CurlyBracketClose,
    Number(i64),
    Symbol(String),
    Word(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::CurlyBracketOpen => write!(f, "{{"),
            Token::CurlyBracketClose => write!(f, "}}"),
            Token::Number(number) => write!(f, "{number}"),
            Token::Symbol(symbol) => write!(f, ":{symbol}"),
            Token::Word(word) => write!(f, "{word}"),
        }
    }
}

#[derive(Debug)]
pub struct AddressedTokens {
    pub left: Option<Address>,
    pub left_to_right: HashMap<Address, AddressedToken>,
    pub right_to_left: HashMap<Address, AddressedToken>,
}

#[derive(Clone, Debug)]
pub struct AddressedToken {
    pub token: Token,
    pub left: Option<Address>,
    pub right: Option<Address>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Address(pub blake3::Hash);

#[derive(Debug, thiserror::Error)]
#[error("No more tokens")]
pub struct NoMoreTokens;
