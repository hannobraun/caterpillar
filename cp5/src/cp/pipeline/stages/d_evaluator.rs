use crate::cp::{
    evaluate::Evaluator,
    pipeline::{
        ir::analyzer_output::AnalyzerEvent, stage_input::StageInputReader,
    },
    Bindings, DataStack, EvaluatorError, Functions, PipelineError,
};

pub fn evaluate(
    mut expressions: StageInputReader<AnalyzerEvent>,
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
