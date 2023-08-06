use crate::value::Value;

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    pub elements: Vec<SyntaxElement>,
}

impl SyntaxTree {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    FnRef(String),
    Value(Value),
}
