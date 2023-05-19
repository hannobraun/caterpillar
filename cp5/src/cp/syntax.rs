pub enum SyntaxElement {
    Block { syntax_tree: Vec<SyntaxElement> },
    Word(String),
}
