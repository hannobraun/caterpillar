pub struct SyntaxTree {
    pub elements: Vec<SyntaxElement>,
}

pub enum SyntaxElement {
    Block { syntax_tree: SyntaxTree },
    Word(String),
}
