use crate::cp::{
    expressions::{Expression, ExpressionGraph},
    functions::Function,
    syntax::{SyntaxElement, SyntaxTree},
    Functions,
};

pub fn analyze(
    syntax_tree: SyntaxTree,
    functions: &mut Functions,
) -> ExpressionGraph {
    let mut expressions = Vec::new();

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

        expressions.push(expression);
    }

    ExpressionGraph::from(expressions)
}
