use std::collections::VecDeque;

use super::keywords::Keyword;

#[derive(Debug)]
pub struct Tokens(pub VecDeque<Token>);

impl Tokens {
    pub fn peek(&self) -> Result<&Token, NoMoreTokens> {
        self.0.front().ok_or(NoMoreTokens)
    }

    pub fn next(&mut self) -> Result<Token, NoMoreTokens> {
        self.0.pop_front().ok_or(NoMoreTokens)
    }

    pub fn expect(&mut self, token: Token) -> Result<Token, ExpectedToken> {
        match self.0.pop_front() {
            Some(next) => {
                if next == token {
                    Ok(next)
                } else {
                    Err(ExpectedToken {
                        expected: token,
                        actual: Some(next),
                    })
                }
            }
            None => Err(ExpectedToken {
                expected: token,
                actual: None,
            }),
        }
    }

    pub fn expect_ident(&mut self) -> Result<String, ExpectedToken> {
        match self.0.pop_front() {
            Some(Token::Ident(ident)) => Ok(ident),
            token => Err(ExpectedToken {
                expected: Token::Ident(String::new()),
                actual: token,
            }),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
#[error("No more tokens")]
pub struct NoMoreTokens;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
#[error("Expected `{expected:?}`, got `{actual:?}`")]
pub struct ExpectedToken {
    pub expected: Token,
    pub actual: Option<Token>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Token {
    BindingOperator,
    Period,
    CurlyBracketOpen,
    CurlyBracketClose,
    RoundBracketOpen,
    RoundBracketClose,
    SquareBracketOpen,
    SquareBracketClose,
    Keyword(Keyword),
    Ident(String),
    Symbol(String),
}

impl Token {
    const EAGER_TOKENS: &[(&'static str, Token)] = &[
        ("=>", Token::BindingOperator),
        (".", Token::Period),
        ("{", Token::CurlyBracketOpen),
        ("}", Token::CurlyBracketClose),
        ("(", Token::RoundBracketOpen),
        (")", Token::RoundBracketClose),
        ("[", Token::SquareBracketOpen),
        ("]", Token::SquareBracketClose),
    ];

    pub fn match_eagerly(s: &str, tokens: &mut Vec<Self>) -> bool {
        for (token_str, token) in Self::EAGER_TOKENS {
            if s == *token_str {
                tokens.push(token.clone());
                return true;
            }

            if let Some((first_token, "")) = s.split_once(token_str) {
                Self::match_delimited(first_token, tokens);
                tokens.push(token.clone());
                return true;
            }
        }

        false
    }
    pub fn match_delimited(s: &str, tokens: &mut Vec<Self>) {
        if Self::match_eagerly(s, tokens) {
            return;
        }
        if let Some(keyword) = Keyword::parse(s) {
            tokens.push(Token::Keyword(keyword));
            return;
        }
        if let Some(("", symbol)) = s.split_once(':') {
            tokens.push(Token::Symbol(symbol.into()));
            return;
        }

        tokens.push(Token::Ident(s.into()));
    }
}
