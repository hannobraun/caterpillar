use std::fmt;

#[derive(Debug)]
pub struct Syntax<'r> {
    expressions: &'r mut Vec<Expression>,
}

impl<'r> Syntax<'r> {
    pub fn new(expressions: &'r mut Vec<Expression>) -> Self {
        Self { expressions }
    }

    pub fn v(&mut self, value: usize) -> &mut Self {
        self.expressions
            .push(Expression::new(ExpressionKind::Value(value)));
        self
    }

    pub fn w(&mut self, name: &str) -> &mut Self {
        self.expressions.push(Expression::new(ExpressionKind::Word {
            name: name.to_string(),
        }));
        self
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub breakpoint: bool,
}

impl Expression {
    pub fn new(kind: ExpressionKind) -> Self {
        Self {
            kind,
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
