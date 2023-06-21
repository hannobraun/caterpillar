use std::convert::Infallible;

use crate::cp::{
    data_stack::Value,
    syntax::{SyntaxElement, SyntaxTree},
    Functions,
};

use super::{stage_input::StageInputReader, PipelineError};

pub fn analyze(
    mut syntax_elements: StageInputReader<SyntaxElement>,
    functions: &mut Functions,
) -> Result<Expression, PipelineError<Infallible>> {
    loop {
        let syntax_element = syntax_elements.read()?;
        let expression = analyze_syntax_element(syntax_element, functions);
        syntax_elements.take();

        match expression {
            Some(expression) => return Ok(expression),
            None => continue,
        }
    }
}

fn analyze_syntax_element(
    syntax_element: &SyntaxElement,
    functions: &mut Functions,
) -> Option<Expression> {
    let expression = match syntax_element {
        SyntaxElement::Array { syntax_tree } => {
            let expressions = analyze_syntax_tree(syntax_tree, functions);
            Expression::Array { expressions }
        }
        SyntaxElement::Binding { idents } => {
            let idents = idents.clone();
            Expression::Binding { idents }
        }
        SyntaxElement::Block { syntax_tree } => {
            let expressions = analyze_syntax_tree(syntax_tree, functions);
            Expression::Value(Value::Block(expressions))
        }
        SyntaxElement::Function { name, body } => {
            let name = name.clone();
            let body = analyze_syntax_tree(body, functions);
            Expression::Function { name, body }
        }
        SyntaxElement::Module { name, body } => {
            let name = name.clone();
            let body = analyze_syntax_tree(body, functions);
            Expression::Module { name, body }
        }
        SyntaxElement::String(s) => {
            let s = s.clone();
            Expression::Value(Value::String(s))
        }
        SyntaxElement::Test { name, body } => {
            let name = name.clone();
            let body = analyze_syntax_tree(body, functions);
            Expression::Test { name, body }
        }
        SyntaxElement::Word(word) => {
            Expression::RawSyntaxElement(SyntaxElement::Word(word.clone()))
        }
    };

    Some(expression)
}

fn analyze_syntax_tree(
    syntax_tree: &SyntaxTree,
    functions: &mut Functions,
) -> Expressions {
    let mut expressions = Expressions {
        elements: Vec::new(),
    };

    for syntax_element in syntax_tree {
        let expression = analyze_syntax_element(syntax_element, functions);
        if let Some(expression) = expression {
            expressions.elements.push(expression);
        }
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
