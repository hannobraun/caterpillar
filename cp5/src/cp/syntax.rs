#[derive(Debug)]
pub struct SyntaxTree {
    pub elements: Vec<SyntaxElement>,
}

#[derive(Debug)]
pub enum SyntaxElement {
    Block { syntax_tree: SyntaxTree },
    Word(String),
}
