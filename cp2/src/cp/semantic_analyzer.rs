use std::{collections::BTreeMap, vec};

use super::{parser::SyntaxElement, SyntaxTree};

#[derive(Clone, Debug)]
pub struct ExpressionGraph(Vec<Expression>);

impl ExpressionGraph {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl IntoIterator for ExpressionGraph {
    type Item = Expression;
    type IntoIter = vec::IntoIter<Expression>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Binding(Vec<String>),
    Array { syntax_tree: ExpressionGraph },
    Block { syntax_tree: ExpressionGraph },
    Word(String),
}

pub type Functions = BTreeMap<String, ExpressionGraph>;

pub fn analyze(syntax_tree: SyntaxTree) -> ExpressionGraph {
    let mut expressions = ExpressionGraph::new();

    for syntax_element in syntax_tree {
        let expression = match syntax_element {
            SyntaxElement::Function { .. } => {
                // not handled yet
                continue;
            }
            SyntaxElement::Binding(binding) => Expression::Binding(binding),
            SyntaxElement::Array { syntax_tree } => {
                let expressions = analyze(syntax_tree);
                Expression::Array {
                    syntax_tree: expressions,
                }
            }
            SyntaxElement::Block { syntax_tree } => {
                let expressions = analyze(syntax_tree);
                Expression::Block {
                    syntax_tree: expressions,
                }
            }
            SyntaxElement::Word(word) => Expression::Word(word),
        };

        expressions.0.push(expression);
    }

    expressions
}
