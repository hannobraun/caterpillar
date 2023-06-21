use std::convert::Infallible;

use crate::cp::syntax::SyntaxElement;

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
            let mut expressions = Expressions {
                elements: Vec::new(),
            };

            for syntax_element in body {
                let expression = analyze_syntax_element(syntax_element);
                expressions.elements.push(expression);
            }

            Expression::Module {
                name: name.clone(),
                body: expressions,
            }
        }
        syntax_element => Expression::RawSyntaxElement(syntax_element.clone()),
    }
}

pub struct Expressions {
    pub elements: Vec<Expression>,
}

pub enum Expression {
    Module { name: String, body: Expressions },
    RawSyntaxElement(SyntaxElement),
}
