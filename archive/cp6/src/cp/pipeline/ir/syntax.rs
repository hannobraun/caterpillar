use std::slice;

use crate::cp::runtime::data_stack::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxTree {
    pub elements: Vec<SyntaxElement>,
}

impl<'r> IntoIterator for &'r SyntaxTree {
    type Item = &'r SyntaxElement;
    type IntoIter = slice::Iter<'r, SyntaxElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntaxElement {
    Array { syntax_tree: SyntaxTree },
    Binding { idents: Vec<String> },
    Block { syntax_tree: SyntaxTree },
    Function { name: String, body: SyntaxTree },
    Module { name: String, body: SyntaxTree },
    Test { name: String, body: SyntaxTree },
    Value(Value),
    Word(String),
}
