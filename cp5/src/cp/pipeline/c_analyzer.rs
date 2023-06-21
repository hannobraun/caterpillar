use std::convert::Infallible;

use crate::cp::{
    data_stack::Value,
    syntax::{SyntaxElement, SyntaxTree},
};

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
        SyntaxElement::Array { syntax_tree } => {
            let expressions = analyze_syntax_tree(syntax_tree);
            Expression::Array { expressions }
        }
        SyntaxElement::Binding { idents } => {
            let idents = idents.clone();
            Expression::Binding { idents }
        }
        SyntaxElement::Block { syntax_tree } => {
            let expressions = analyze_syntax_tree(syntax_tree);
            Expression::Value(Value::Block(expressions))
        }
        SyntaxElement::Function { name, body } => {
            let name = name.clone();
            let body = analyze_syntax_tree(body);
            Expression::Function { name, body }
        }
        SyntaxElement::Module { name, body } => {
            let name = name.clone();
            let body = analyze_syntax_tree(body);
            Expression::Module { name, body }
        }
        SyntaxElement::String(s) => {
            let s = s.clone();
            Expression::Value(Value::String(s))
        }
        SyntaxElement::Test { name, body } => {
            let name = name.clone();
            let body = analyze_syntax_tree(body);
            Expression::Test { name, body }
        }
        SyntaxElement::Word(word) => {
            Expression::RawSyntaxElement(SyntaxElement::Word(word.clone()))
        }
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expressions {
    pub elements: Vec<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Array { expressions: Expressions },
    Binding { idents: Vec<String> },
    Function { name: String, body: Expressions },
    Module { name: String, body: Expressions },
    Test { name: String, body: Expressions },
    Value(Value),
    RawSyntaxElement(SyntaxElement),
}
