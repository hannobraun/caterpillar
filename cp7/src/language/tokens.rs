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
    pub left: Option<TokenAddressRight>,
    pub left_to_right: HashMap<TokenAddressRight, AddressedToken>,
    pub right_to_left: HashMap<TokenAddressLeft, AddressedToken>,
}

impl Tokens {
    pub fn left_to_right(&self) -> TokensLeftToRight {
        TokensLeftToRight {
            current: self.left,
            tokens: self,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TokenAddressRight {
    pub hash: blake3::Hash,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TokenAddressLeft {
    pub hash: blake3::Hash,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddressedToken {
    pub token: Token,
    pub left: Option<TokenAddressLeft>,
    pub right: Option<TokenAddressRight>,
}

impl AddressedToken {
    pub fn hash(&self) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();

        if let Some(left) = self.left {
            hasher.update(left.hash.as_bytes());
        }
        if let Some(right) = self.right {
            hasher.update(right.hash.as_bytes());
        }

        hasher.finalize()
    }
}

pub struct TokensLeftToRight<'r> {
    current: Option<TokenAddressRight>,
    tokens: &'r Tokens,
}

impl TokensLeftToRight<'_> {
    pub fn peek(&self) -> Option<&AddressedToken> {
        let current = self.current?;
        self.tokens.left_to_right.get(&current)
    }

    pub fn next(&mut self) -> Option<&AddressedToken> {
        let current = self.current?;
        let token = self.tokens.left_to_right.get(&current)?;
        self.current = token.right;
        Some(token)
    }
}
