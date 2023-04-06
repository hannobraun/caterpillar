use std::vec;

#[derive(Clone, Debug)]
pub struct SyntaxTree(Vec<SyntaxElement>);

impl From<Vec<SyntaxElement>> for SyntaxTree {
    fn from(syntax_tree: Vec<SyntaxElement>) -> Self {
        Self(syntax_tree)
    }
}

impl IntoIterator for SyntaxTree {
    type Item = SyntaxElement;
    type IntoIter = vec::IntoIter<SyntaxElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    Function {
        name: String,
        body: SyntaxTree,
    },

    /// Binds values from the stack to provided names
    Binding(Vec<String>),

    Array {
        syntax_tree: SyntaxTree,
    },

    /// A block of code that is lazily evaluated
    Block {
        syntax_tree: SyntaxTree,
    },

    /// A word refers to a function or variable
    Word(String),
}
