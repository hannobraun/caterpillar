#[derive(Debug)]
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

#[derive(Debug)]
pub enum SyntaxElement {
    Block(SyntaxTree),
    Number(i64),
    Symbol(String),
    Word(String),
}
