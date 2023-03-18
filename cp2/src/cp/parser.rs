use super::tokenizer::{Token, Tokens};

pub struct Expressions(pub Vec<Expression>);

pub enum Expression {
    Word(String),
}

pub fn parse(tokens: Tokens) -> Expressions {
    let expressions = tokens
        .0
        .into_iter()
        .map(|Token::Word(word)| Expression::Word(word))
        .collect();
    Expressions(expressions)
}
