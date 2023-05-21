#[derive(Clone, Debug)]
pub struct SyntaxTree {
    pub elements: Vec<SyntaxElement>,
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    Block { syntax_tree: SyntaxTree },
    Word(String),
}
