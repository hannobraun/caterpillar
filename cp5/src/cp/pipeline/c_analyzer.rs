use std::convert::Infallible;

use crate::cp::{
    data_stack::Value,
    functions::Module,
    syntax::{SyntaxElement, SyntaxTree},
    Bindings, Functions,
};

use super::{stage_input::StageInputReader, PipelineError};

pub fn analyze(
    mut syntax_elements: StageInputReader<SyntaxElement>,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<Expression, PipelineError<Infallible>> {
    loop {
        let syntax_element = syntax_elements.read()?;
        let expression = analyze_syntax_element(
            syntax_element,
            Module::none(),
            bindings,
            functions,
            tests,
        );
        syntax_elements.take();

        match expression {
            Some(expression) => return Ok(expression),
            None => continue,
        }
    }
}

fn analyze_syntax_element(
    syntax_element: &SyntaxElement,
    module: Module,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Option<Expression> {
    let expression = match syntax_element {
        SyntaxElement::Array { syntax_tree } => {
            let expressions = analyze_syntax_tree(
                syntax_tree,
                module,
                bindings,
                functions,
                tests,
            );
            Expression::Array { expressions }
        }
        SyntaxElement::Binding { idents } => {
            for ident in idents {
                bindings.declare(ident.clone());
            }

            let idents = idents.clone();
            Expression::Binding { idents }
        }
        SyntaxElement::Block { syntax_tree } => {
            let expressions = analyze_syntax_tree(
                syntax_tree,
                module,
                bindings,
                functions,
                tests,
            );
            Expression::Value(Value::Block(expressions))
        }
        SyntaxElement::Function { name, body } => {
            functions.declare(name.clone());

            let body = analyze_syntax_tree(
                body,
                Module::none(),
                bindings,
                functions,
                tests,
            );
            functions.define(Module::none(), name.clone(), body);

            return None;
        }
        SyntaxElement::Module { name, body } => {
            let name = name.clone();
            let body = analyze_syntax_tree(
                body,
                Module::some(&name),
                bindings,
                functions,
                tests,
            );
            Expression::Module { name, body }
        }
        SyntaxElement::Test { name, body } => {
            let name = name.clone();
            let body =
                analyze_syntax_tree(body, module, bindings, functions, tests);

            tests.define(module, name, body);

            return None;
        }
        SyntaxElement::Value(value) => {
            let s = value.clone();
            Expression::Value(s)
        }
        SyntaxElement::Word(word) => {
            let refers_to_binding = bindings.is_declared(word);
            let refers_to_function = functions.is_declared(word);

            if refers_to_binding {
                return Some(Expression::EvalBinding { name: word.clone() });
            }
            if refers_to_function {
                return Some(Expression::EvalFunction { name: word.clone() });
            }

            Expression::RawSyntaxElement(SyntaxElement::Word(word.clone()))
        }
    };

    Some(expression)
}

fn analyze_syntax_tree(
    syntax_tree: &SyntaxTree,
    module: Module,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Expressions {
    let mut expressions = Expressions {
        elements: Vec::new(),
    };

    for syntax_element in syntax_tree {
        let expression = analyze_syntax_element(
            syntax_element,
            module,
            bindings,
            functions,
            tests,
        );
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
    EvalBinding { name: String },
    EvalFunction { name: String },
    Module { name: String, body: Expressions },
    Value(Value),
    RawSyntaxElement(SyntaxElement),
}
