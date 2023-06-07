use std::collections::BTreeMap;

use crate::cp::{
    data_stack::Value, functions::Module, syntax::SyntaxElement, DataStack,
    DataStackError, Functions, StageInput,
};

use super::{stage_input::StageInputReader, PipelineError};

pub fn evaluate_all(
    mut syntax_elements: StageInput<SyntaxElement>,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), PipelineError<EvaluatorError>> {
    while !syntax_elements.is_empty() {
        evaluate(
            syntax_elements.reader(),
            data_stack,
            bindings,
            functions,
            tests,
        )?;
    }

    Ok(())
}

pub fn evaluate(
    mut syntax_elements: StageInputReader<SyntaxElement>,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), PipelineError<EvaluatorError>> {
    let syntax_element = syntax_elements.next()?;
    evaluate_syntax_element(
        Module::none(),
        syntax_element,
        data_stack,
        bindings,
        functions,
        tests,
    )?;
    syntax_elements.take();
    Ok(())
}

fn evaluate_syntax_element(
    module: Module,
    syntax_element: &SyntaxElement,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), PipelineError<EvaluatorError>> {
    match syntax_element {
        SyntaxElement::Block { syntax_tree } => {
            data_stack.push(Value::Block(syntax_tree.clone()));
            Ok(())
        }
        SyntaxElement::Function { name, body } => {
            functions.define(Module::none(), name.clone(), body.clone());
            Ok(())
        }
        SyntaxElement::Module { name, body } => {
            for syntax_element in &body.elements {
                evaluate_syntax_element(
                    Module::some(name),
                    syntax_element,
                    data_stack,
                    bindings,
                    functions,
                    tests,
                )?;
            }
            Ok(())
        }
        SyntaxElement::Test { name, body } => {
            tests.define(module, name.clone(), body.clone());
            Ok(())
        }
        SyntaxElement::Binding { idents } => {
            for ident in idents.iter().rev() {
                let value = data_stack.pop_any()?;
                bindings.inner.insert(ident.clone(), value);
            }
            Ok(())
        }
        SyntaxElement::String(s) => {
            data_stack.push(s.clone());
            Ok(())
        }
        SyntaxElement::Word(word) => {
            evaluate_word(module, word, data_stack, bindings, functions, tests)
        }
    }
}

fn evaluate_word(
    module: Module,
    word: &str,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), PipelineError<EvaluatorError>> {
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
        "eval" => {
            let block = data_stack.pop_block()?;
            for syntax_element in block.elements {
                evaluate_syntax_element(
                    module,
                    &syntax_element,
                    data_stack,
                    bindings,
                    functions,
                    tests,
                )?;
            }
        }
        "=" => {
            let a = data_stack.pop_string()?;
            let b = data_stack.pop_string()?;

            let eq = a == b;

            data_stack.push(eq);
        }
        _ => {
            if let Some(value) = bindings.inner.remove(word) {
                data_stack.push(value);
                return Ok(());
            }

            if let Some(function) = functions.get(word) {
                for syntax_element in function.body.elements {
                    evaluate_syntax_element(
                        module,
                        &syntax_element,
                        data_stack,
                        bindings,
                        functions,
                        tests,
                    )?;
                }
                return Ok(());
            }

            return Err(PipelineError::Stage(EvaluatorError::UnknownWord(
                word.into(),
            )));
        }
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
pub enum EvaluatorError {
    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}

impl From<DataStackError> for PipelineError<EvaluatorError> {
    fn from(err: DataStackError) -> Self {
        PipelineError::Stage(err.into())
    }
}
