use super::Token;

pub type Expressions = Vec<Expression>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Expression {
    Block(Expressions),
    List(Expressions),
    Fn(String),
}

pub fn parse(tokens: impl IntoIterator<Item = Token>) -> Expressions {
    let tokens = tokens.into_iter();
    parse_expressions(tokens, None)
}

fn parse_expressions(
    mut tokens: impl Iterator<Item = Token>,
    terminator: Option<&Token>,
) -> Expressions {
    let mut expressions = Vec::new();

    while let Some(expression) = parse_expression(&mut tokens, terminator) {
        expressions.push(expression);
    }

    expressions
}

fn parse_expression(
    tokens: &mut dyn Iterator<Item = Token>,
    terminator: Option<&Token>,
) -> Option<Expression> {
    let token = tokens.next()?;

    if Some(&token) == terminator {
        return None;
    }

    match token {
        Token::Fn(name) => Some(Expression::Fn(name)),
        Token::Name(name) => todo!("Unexpected name `{name}`"),
        Token::BlockOpen => {
            let expressions =
                parse_expressions(tokens, Some(&Token::BlockClose));
            Some(Expression::Block(expressions))
        }
        Token::BlockClose => {
            panic!("Unexpected `}}`")
        }
        Token::ListOpen => {
            let expressions =
                parse_expressions(tokens, Some(&Token::ListClose));
            Some(Expression::List(expressions))
        }
        Token::ListClose => {
            panic!("Unexpected `]`")
        }
    }
}
