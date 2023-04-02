use std::{collections::BTreeMap, vec};

use crate::cp::syntax::{SyntaxElement, SyntaxTree};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Expression {
    Binding(Vec<String>),
    Array { syntax_tree: ExpressionGraph },
    Block { syntax_tree: ExpressionGraph },
    Word(String),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Functions {
    pub registry: BTreeMap<String, Function>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            registry: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Function {
    pub body: ExpressionGraph,
}

pub fn analyze(
    syntax_tree: SyntaxTree,
    functions: &mut Functions,
) -> ExpressionGraph {
    let mut expressions = ExpressionGraph::new();

    for syntax_element in syntax_tree {
        let expression = match syntax_element {
            SyntaxElement::Function { name, body } => {
                let body = analyze(body, functions);
                let function = Function { body };

                functions.registry.insert(name, function);

                continue;
            }
            SyntaxElement::Binding(binding) => Expression::Binding(binding),
            SyntaxElement::Array { syntax_tree } => {
                let expressions = analyze(syntax_tree, functions);
                Expression::Array {
                    syntax_tree: expressions,
                }
            }
            SyntaxElement::Block { syntax_tree } => {
                let expressions = analyze(syntax_tree, functions);
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
