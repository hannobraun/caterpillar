use std::collections::VecDeque;

use enum_variant_type::EnumVariantType;

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

impl From<VecDeque<Token>> for Tokens {
    fn from(tokens: VecDeque<Token>) -> Self {
        Self { inner: tokens }
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

#[derive(Debug, thiserror::Error)]
#[error("No more tokens")]
pub struct NoMoreTokens;
