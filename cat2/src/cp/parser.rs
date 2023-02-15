use super::Token;

pub type Expressions = Vec<Expression>;

pub enum Expression {
    Fn(String),
}

pub fn parse(tokens: impl IntoIterator<Item = Token>) -> Expressions {
    let mut tokens = tokens.into_iter();
    let mut expressions = Vec::new();

    while let Some(expression) = parse_expression(&mut tokens) {
        expressions.push(expression);
    }

    expressions
}

fn parse_expression(
    mut tokens: impl Iterator<Item = Token>,
) -> Option<Expression> {
    match tokens.next() {
        Some(Token::Fn(name)) => Some(Expression::Fn(name)),
        Some(Token::BlockOpen) => {
            // Currently ignored.
            None
        }
        Some(Token::BlockClose) => {
            // Currently ignored.
            None
        }
        None => None,
    }
}
