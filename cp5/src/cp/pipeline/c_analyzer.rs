use std::convert::Infallible;

use crate::cp::syntax::{SyntaxElement, SyntaxTree};

use super::{stage_input::StageInputReader, PipelineError};

pub fn analyze(
    mut syntax_elements: StageInputReader<SyntaxElement>,
) -> Result<Expression, PipelineError<Infallible>> {
    let syntax_element = syntax_elements.read()?;
    let expression = analyze_syntax_element(syntax_element);
    syntax_elements.take();
    Ok(expression)
}

fn analyze_syntax_element(syntax_element: &SyntaxElement) -> Expression {
    match syntax_element {
        SyntaxElement::Module { name, body } => {
            let expressions = analyze_syntax_tree(body);
            Expression::Module {
                name: name.clone(),
                body: expressions,
            }
        }
        syntax_element => Expression::RawSyntaxElement(syntax_element.clone()),
    }
}

fn analyze_syntax_tree(syntax_tree: &SyntaxTree) -> Expressions {
    let mut expressions = Expressions {
        elements: Vec::new(),
    };

    for syntax_element in syntax_tree {
        let expression = analyze_syntax_element(syntax_element);
        expressions.elements.push(expression);
    }

    expressions
}

#[derive(Clone, Debug)]
pub struct Expressions {
    pub elements: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Module { name: String, body: Expressions },
    RawSyntaxElement(SyntaxElement),
}
