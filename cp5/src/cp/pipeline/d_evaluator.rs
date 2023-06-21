use std::collections::BTreeMap;

use crate::cp::{
    data_stack::{Array, Value},
    functions::Module,
    syntax::SyntaxElement,
    DataStack, DataStackError, Expression, Functions, StageInput,
};

use super::{
    c_analyzer::Expressions, stage_input::StageInputReader, PipelineError,
};

pub fn evaluate_all(
    mut expressions: StageInput<Expression>,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), PipelineError<EvaluatorError>> {
    while !expressions.is_empty() {
        evaluate(expressions.reader(), data_stack, bindings, functions, tests)?;
    }

    Ok(())
}

pub fn evaluate(
    mut expressions: StageInputReader<Expression>,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), PipelineError<EvaluatorError>> {
    let expression = expressions.read()?;
    evaluate_expression(
        Module::none(),
        expression,
        data_stack,
        bindings,
        functions,
        tests,
    )
    .map_err(|err| match err {
        PipelineError::NotEnoughInput(err) => {
            PipelineError::NotEnoughInput(err)
        }
        PipelineError::Stage(kind) => {
            PipelineError::Stage(EvaluatorError { kind })
        }
    })?;
    expressions.take();
    Ok(())
}

fn evaluate_expression(
    module: Module,
    expression: &Expression,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), PipelineError<EvaluatorErrorKind>> {
    match expression {
        Expression::RawSyntaxElement(SyntaxElement::Array { syntax_tree }) => {
            data_stack.mark();

            for syntax_element in &syntax_tree.elements {
                evaluate_expression(
                    module,
                    &Expression::RawSyntaxElement(syntax_element.clone()),
                    data_stack,
                    bindings,
                    functions,
                    tests,
                )?;
            }

            let array = Array {
                elements: data_stack.drain_values_from_marker().collect(),
            };
            let array = Value::Array(array);
            data_stack.push(array);
        }
        Expression::RawSyntaxElement(SyntaxElement::Block { syntax_tree }) => {
            data_stack.push(Value::Block(syntax_tree.clone()));
        }
        Expression::RawSyntaxElement(SyntaxElement::Function {
            name,
            body,
        }) => {
            let body = Expressions {
                elements: body
                    .elements
                    .iter()
                    .cloned()
                    .map(|syntax_element| {
                        Expression::RawSyntaxElement(syntax_element)
                    })
                    .collect(),
            };
            functions.define(Module::none(), name.clone(), body);
        }
        Expression::Module { name, body } => {
            for expression in &body.elements {
                evaluate_expression(
                    Module::some(name),
                    expression,
                    data_stack,
                    bindings,
                    functions,
                    tests,
                )?;
            }
        }
        Expression::RawSyntaxElement(SyntaxElement::Test { name, body }) => {
            let body = Expressions {
                elements: body
                    .elements
                    .iter()
                    .cloned()
                    .map(|syntax_element| {
                        Expression::RawSyntaxElement(syntax_element)
                    })
                    .collect(),
            };
            tests.define(module, name.clone(), body);
        }
        Expression::RawSyntaxElement(SyntaxElement::Binding { idents }) => {
            for ident in idents.iter().rev() {
                let value = data_stack.pop_any()?;
                bindings.inner.insert(ident.clone(), value);
            }
        }
        Expression::RawSyntaxElement(SyntaxElement::String(s)) => {
            data_stack.push(s.clone());
        }
        Expression::RawSyntaxElement(SyntaxElement::Word(word)) => {
            evaluate_word(
                module, word, data_stack, bindings, functions, tests,
            )?;
        }
        Expression::RawSyntaxElement(syntax_element) => {
            panic!("Unexpected raw syntax element: {syntax_element:?}")
        }
    }

    Ok(())
}

fn evaluate_word(
    module: Module,
    word: &str,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), PipelineError<EvaluatorErrorKind>> {
    match word {
        "clone" => {
            let a = data_stack.pop_any()?;

            data_stack.push(a.clone());
            data_stack.push(a);
        }
        "drop" => {
            data_stack.pop_any()?;
        }
        "true" => data_stack.push(true),
        "false" => data_stack.push(false),
        "and" => {
            let a = data_stack.pop_bool()?;
            let b = data_stack.pop_bool()?;

            data_stack.push(a && b);
        }
        "not" => {
            let b = data_stack.pop_bool()?;
            data_stack.push(!b);
        }
        "if" => {
            let else_ = data_stack.pop_block()?;
            let then_ = data_stack.pop_block()?;
            let cond = data_stack.pop_bool()?;

            let block = if cond { then_ } else { else_ };

            let block = Expressions {
                elements: block
                    .elements
                    .into_iter()
                    .map(|syntax_element| {
                        Expression::RawSyntaxElement(syntax_element)
                    })
                    .collect(),
            };

            evaluate_block(
                module, block, data_stack, bindings, functions, tests,
            )?;
        }
        "unwrap" => {
            let array = data_stack.pop_array()?;

            for value in array.elements {
                data_stack.push(value);
            }
        }
        "eval" => {
            let block = data_stack.pop_block()?;
            let block = Expressions {
                elements: block
                    .elements
                    .into_iter()
                    .map(|syntax_element| {
                        Expression::RawSyntaxElement(syntax_element)
                    })
                    .collect(),
            };
            evaluate_block(
                module, block, data_stack, bindings, functions, tests,
            )?;
        }
        "=" => {
            let b = data_stack.pop_any()?;
            let a = data_stack.pop_any()?;

            let eq = a == b;

            data_stack.push(eq);
        }
        "-" => {
            let b = data_stack.pop_u8()?;
            let a = data_stack.pop_u8()?;

            data_stack.push(a - b);
        }
        _ => {
            if let Some(value) = bindings.inner.get(word) {
                data_stack.push(value.clone());
                return Ok(());
            }

            if let Some(function) = functions.get(word) {
                evaluate_block(
                    module,
                    function.body,
                    data_stack,
                    bindings,
                    functions,
                    tests,
                )?;
                return Ok(());
            }

            if let Ok(value) = word.parse::<u8>() {
                data_stack.push(value);
                return Ok(());
            }

            return Err(PipelineError::Stage(EvaluatorErrorKind::UnknownWord(
                word.into(),
            )));
        }
    }

    Ok(())
}

fn evaluate_block(
    module: Module,
    block: Expressions,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), PipelineError<EvaluatorErrorKind>> {
    for expression in block.elements {
        evaluate_expression(
            module,
            &expression,
            data_stack,
            bindings,
            functions,
            tests,
        )?;
    }

    Ok(())
}

pub struct Bindings {
    inner: BTreeMap<String, Value>,
}

impl Bindings {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{kind}")]
pub struct EvaluatorError {
    pub kind: EvaluatorErrorKind,
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorErrorKind {
    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}

impl From<DataStackError> for PipelineError<EvaluatorErrorKind> {
    fn from(err: DataStackError) -> Self {
        PipelineError::Stage(err.into())
    }
}
