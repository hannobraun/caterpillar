#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxTree {
    pub elements: Vec<SyntaxElement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntaxElement {
    Array { syntax_tree: SyntaxTree },
    Block { syntax_tree: SyntaxTree },
    Function { name: String, body: SyntaxTree },
    Module { name: String, body: SyntaxTree },
    Test { name: String, body: SyntaxTree },
    Binding { idents: Vec<String> },
    String(String),
    Word(String),
}
