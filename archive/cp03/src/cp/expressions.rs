use std::vec;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct ExpressionGraph(Vec<Expression>);

impl From<Vec<Expression>> for ExpressionGraph {
    fn from(expressions: Vec<Expression>) -> Self {
        Self(expressions)
    }
}

impl IntoIterator for ExpressionGraph {
    type Item = Expression;
    type IntoIter = vec::IntoIter<Expression>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Expression {
    Binding(Vec<String>),
    Array { syntax_tree: ExpressionGraph },
    Block { syntax_tree: ExpressionGraph },
    String(String),
    Word(String),
}
