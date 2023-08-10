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
pub struct AddressedTokens {
    pub left: Option<Address>,
    pub left_to_right: HashMap<Address, AddressedToken>,
    pub right_to_left: HashMap<Address, AddressedToken>,
}

impl AddressedTokens {
    pub fn iter(&mut self) -> TokenIter {
        TokenIter {
            current: self.left,
            tokens: self,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddressedToken {
    pub token: Token,
    pub left: Option<Address>,
    pub right: Option<Address>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Address(pub blake3::Hash);

pub struct TokenIter<'r> {
    current: Option<Address>,
    tokens: &'r AddressedTokens,
}

impl TokenIter<'_> {
    pub fn peek(&self) -> Option<&Token> {
        let current = self.current?;
        self.tokens
            .right_to_left
            .get(&current)
            .map(|token| &token.token)
    }

    pub fn next(&mut self) -> Option<Token> {
        let current = self.current?;
        let token = self.tokens.right_to_left.get(&current).cloned()?;
        self.current = token.right;
        Some(token.token)
    }
}
