use crate::cp::{
    evaluate::Evaluator, Bindings, DataStack, EvaluatorError, Functions,
};

use super::{
    c_analyzer::Expression, stage_input::StageInputReader, PipelineError,
};

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
