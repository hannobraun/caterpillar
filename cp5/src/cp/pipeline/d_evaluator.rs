use crate::cp::{
    data_stack::{Array, Value},
    functions::Function,
    syntax::SyntaxElement,
    Bindings, DataStack, EvaluatorError, Expression, Functions, StageInput,
};

use super::{
    c_analyzer::Expressions, stage_input::StageInputReader, PipelineError,
};

pub fn evaluate_all(
    mut expressions: StageInput<Expression>,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &Functions,
    tests: &Functions,
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
    functions: &Functions,
    tests: &Functions,
) -> Result<(), PipelineError<EvaluatorError>> {
    let expression = expressions.read()?;
    evaluate_expression(expression, data_stack, bindings, functions, tests)
        .map_err(PipelineError::Stage)?;
    expressions.take();
    Ok(())
}

fn evaluate_expression(
    expression: &Expression,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &Functions,
    tests: &Functions,
) -> Result<(), EvaluatorError> {
    match expression {
        Expression::Array { expressions } => {
            data_stack.mark();

            for expression in &expressions.elements {
                evaluate_expression(
                    expression, data_stack, bindings, functions, tests,
                )?;
            }

            let array = Array {
                elements: data_stack.drain_values_from_marker().collect(),
            };
            let array = Value::Array(array);
            data_stack.push(array);
        }
        Expression::Binding { idents } => {
            for ident in idents.iter().rev() {
                let value = data_stack.pop_any()?;
                bindings.inner.insert(ident.clone(), value);
            }
        }
        Expression::EvalFunction { name } => {
            evaluate_word(name, data_stack, bindings, functions, tests)?;
        }
        Expression::Module { body, .. } => {
            for expression in &body.elements {
                evaluate_expression(
                    expression, data_stack, bindings, functions, tests,
                )?;
            }
        }
        Expression::Value(value) => {
            data_stack.push(value.clone());
        }
        Expression::RawSyntaxElement(SyntaxElement::Word(word)) => {
            evaluate_word(word, data_stack, bindings, functions, tests)?;
        }
        Expression::RawSyntaxElement(syntax_element) => {
            panic!("Unexpected raw syntax element: {syntax_element:?}")
        }
    }

    Ok(())
}

fn evaluate_word(
    word: &str,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &Functions,
    tests: &Functions,
) -> Result<(), EvaluatorError> {
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

            evaluate_block(block, data_stack, bindings, functions, tests)?;
        }
        "unwrap" => {
            let array = data_stack.pop_array()?;

            for value in array.elements {
                data_stack.push(value);
            }
        }
        "eval" => {
            let block = data_stack.pop_block()?;
            evaluate_block(block, data_stack, bindings, functions, tests)?;
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
                let Function::UserDefined { body, .. } = function;
                evaluate_block(body, data_stack, bindings, functions, tests)?;
                return Ok(());
            }

            if let Ok(value) = word.parse::<u8>() {
                data_stack.push(value);
                return Ok(());
            }

            return Err(EvaluatorError::UnknownWord(word.into()));
        }
    }

    Ok(())
}

fn evaluate_block(
    block: Expressions,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &Functions,
    tests: &Functions,
) -> Result<(), EvaluatorError> {
    for expression in block.elements {
        evaluate_expression(
            &expression,
            data_stack,
            bindings,
            functions,
            tests,
        )?;
    }

    Ok(())
}
