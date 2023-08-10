use std::{collections::HashMap, fmt};

use enum_variant_type::EnumVariantType;

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
pub struct Tokens {
    pub left: Option<TokenAddress>,
    pub left_to_right: HashMap<TokenAddress, AddressedToken>,
    pub right_to_left: HashMap<TokenAddress, AddressedToken>,
}

impl Tokens {
    pub fn iter(&mut self) -> TokenIter {
        TokenIter {
            current: self.left,
            tokens: self,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TokenAddress(pub blake3::Hash);

#[derive(Clone, Debug)]
pub struct AddressedToken {
    pub token: Token,
    pub left: Option<TokenAddress>,
    pub right: Option<TokenAddress>,
}

pub struct TokenIter<'r> {
    current: Option<TokenAddress>,
    tokens: &'r Tokens,
}

impl TokenIter<'_> {
    pub fn peek(&self) -> Option<AddressedToken> {
        let current = self.current?;
        self.tokens.right_to_left.get(&current).cloned()
    }

    pub fn next(&mut self) -> Option<AddressedToken> {
        let token = self.peek()?;
        self.current = token.right;
        Some(token)
    }
}
