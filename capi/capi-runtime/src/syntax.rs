use std::fmt;

use crate::SourceLocation;

#[derive(Debug)]
pub struct Syntax<'r> {
    next_location: SourceLocation,
    expressions: &'r mut Vec<Expression>,
}

impl<'r> Syntax<'r> {
    pub fn new(function: String, expressions: &'r mut Vec<Expression>) -> Self {
        Self {
            next_location: SourceLocation::first_in_function(function),
            expressions,
        }
    }

    pub fn v(&mut self, value: usize) -> &mut Self {
        let location = self.next_location.increment();
        self.expressions
            .push(Expression::new(ExpressionKind::Value(value), location));
        self
    }

    pub fn w(&mut self, name: &str) -> &mut Self {
        let location = self.next_location.increment();
        self.expressions.push(Expression::new(
            ExpressionKind::Word {
                name: name.to_string(),
            },
            location,
        ));
        self
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub location: SourceLocation,
    pub breakpoint: bool,
}

impl Expression {
    pub fn new(kind: ExpressionKind, location: SourceLocation) -> Self {
        Self {
            kind,
            location,
            breakpoint: false,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum ExpressionKind {
    Value(usize),
    Word { name: String },
}

impl fmt::Display for ExpressionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpressionKind::Value(value) => write!(f, "{value}"),
            ExpressionKind::Word { name } => write!(f, "{name}"),
        }
    }
}
