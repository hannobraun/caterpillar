#[derive(Clone, Debug)]
pub struct SyntaxTree {
    pub elements: Vec<SyntaxElement>,
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    Block { syntax_tree: SyntaxTree },
    Function { name: String, body: SyntaxTree },
    Module { name: String, body: SyntaxTree },
    Test { name: String, body: SyntaxTree },
    Binding { idents: Vec<String> },
    String(String),
    Word(String),
}
