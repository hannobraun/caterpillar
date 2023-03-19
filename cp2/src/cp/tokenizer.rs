use std::collections::VecDeque;

pub struct Tokens(pub VecDeque<Token>);

impl Tokens {
    pub fn next(&mut self) -> Result<Token, NoMoreTokens> {
        self.0.pop_front().ok_or(NoMoreTokens)
    }

    #[allow(unused)]
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
}

#[derive(Debug, thiserror::Error)]
#[error("No more tokens")]
pub struct NoMoreTokens;

#[derive(Debug, thiserror::Error)]
#[error("Expected `{expected:?}`, got `{actual:?}`")]
pub struct ExpectedToken {
    pub expected: Token,
    pub actual: Option<Token>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    BindingOperator,
    Period,
    CurlyBracketOpen,
    CurlyBracketClose,
    RoundBracketOpen,
    RoundBracketClose,
    SquareBracketOpen,
    Ident(String),
}

pub fn tokenize(code: &str) -> Tokens {
    let tokens = code
        .split_whitespace()
        .map(|token| match token {
            "=>" => Token::BindingOperator,
            "." => Token::Period,
            "{" => Token::CurlyBracketOpen,
            "}" => Token::CurlyBracketClose,
            "(" => Token::RoundBracketOpen,
            ")" => Token::RoundBracketClose,
            "[" => Token::SquareBracketOpen,
            token => Token::Ident(token.into()),
        })
        .collect();
    Tokens(tokens)
}
