use crate::cp::{
    evaluate::Evaluator, Bindings, DataStack, EvaluatorError, Expression,
    Functions, StageInput,
};

use super::{stage_input::StageInputReader, PipelineError};

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

    let mut evaluator = Evaluator {
        data_stack,
        bindings,
        functions,
        tests,
    };
    evaluator
        .evaluate_expression(expression)
        .map_err(PipelineError::Stage)?;

    expressions.take();
    Ok(())
}
