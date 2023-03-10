use std::fmt;

use super::Token;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Expressions {
    pub inner: Vec<Expression>,
}

impl fmt::Display for Expressions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, expression) in self.inner.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{expression}")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Expression {
    Block(Expressions),
    List(Expressions),
    Fn(String),
    Name(String),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Block(block) => write!(f, "{{ {block} }}"),
            Expression::List(list) => write!(f, "[ {list} ]"),
            Expression::Fn(fn_name) => write!(f, "{fn_name}"),
            Expression::Name(name) => write!(f, ":{name}"),
        }
    }
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

    Expressions { inner: expressions }
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
        Token::Name(name) => Some(Expression::Name(name)),
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
