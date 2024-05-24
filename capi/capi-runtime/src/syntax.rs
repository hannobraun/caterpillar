use std::fmt;

use crate::Value;

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

    pub fn c(&mut self, text: &str) -> &mut Self {
        self.push_expression(ExpressionKind::Comment { text: text.into() })
    }

    pub fn v(&mut self, value: impl Into<Value>) -> &mut Self {
        self.push_expression(ExpressionKind::Value(value.into()))
    }

    pub fn w(&mut self, name: &str) -> &mut Self {
        self.push_expression(ExpressionKind::Word { name: name.into() })
    }

    fn push_expression(&mut self, kind: ExpressionKind) -> &mut Self {
        let location = self.next_location.increment();
        self.expressions.push(Expression::new(kind, location));
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub location: SourceLocation,
}

impl Expression {
    pub fn new(kind: ExpressionKind, location: SourceLocation) -> Self {
        Self { kind, location }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ExpressionKind {
    Comment { text: String },
    Value(Value),
    Word { name: String },
}

impl fmt::Display for ExpressionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpressionKind::Comment { text } => writeln!(f, "# {text}"),
            ExpressionKind::Value(value) => write!(f, "{value}"),
            ExpressionKind::Word { name } => write!(f, "{name}"),
        }
    }
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct SourceLocation {
    function: String,
    index: u32,
}

impl SourceLocation {
    pub fn function(&self) -> &str {
        &self.function
    }

    pub fn first_in_function(function: String) -> Self {
        Self { function, index: 0 }
    }

    pub fn increment(&mut self) -> Self {
        let self_ = self.clone();
        self.index += 1;
        self_
    }
}
