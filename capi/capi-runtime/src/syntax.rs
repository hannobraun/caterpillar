use std::fmt;

#[derive(Debug)]
pub struct Syntax<'r> {
    elements: &'r mut Vec<SyntaxElement>,
}

impl<'r> Syntax<'r> {
    pub fn new(elements: &'r mut Vec<SyntaxElement>) -> Self {
        Self { elements }
    }

    pub fn v(&mut self, value: usize) -> &mut Self {
        self.elements.push(SyntaxElement {
            kind: SyntaxElementKind::Value(value),
        });
        self
    }

    pub fn w(&mut self, name: &str) -> &mut Self {
        self.elements.push(SyntaxElement {
            kind: SyntaxElementKind::Word {
                name: name.to_string(),
            },
        });
        self
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SyntaxElement {
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
