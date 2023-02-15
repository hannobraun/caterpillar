use super::{Token, Tokens};

pub type Expressions = Vec<Expression>;

pub enum Expression {
    Fn(String),
}

pub fn parse(tokens: Tokens) -> Expressions {
    tokens.into_iter().filter_map(parse_token).collect()
}

fn parse_token(token: Token) -> Option<Expression> {
    match token {
        Token::Fn(name) => Some(Expression::Fn(name)),
        Token::BlockOpen => {
            // Currently ignored.
            None
        }
        Token::BlockClose => {
            // Currently ignored.
            None
        }
    }
}
