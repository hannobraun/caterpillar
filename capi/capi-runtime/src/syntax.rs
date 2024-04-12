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
        self.expressions.push(Expression {
            kind: SyntaxElementKind::Value(value),
        });
        self
    }

    pub fn w(&mut self, name: &str) -> &mut Self {
        self.expressions.push(Expression {
            kind: SyntaxElementKind::Word {
                name: name.to_string(),
            },
        });
        self
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Expression {
    pub kind: SyntaxElementKind,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum SyntaxElementKind {
    Value(usize),
    Word { name: String },
}

impl fmt::Display for SyntaxElementKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SyntaxElementKind::Value(value) => write!(f, "{value}"),
            SyntaxElementKind::Word { name } => write!(f, "{name}"),
        }
    }
}
