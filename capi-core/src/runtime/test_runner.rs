use crate::platform::Platform;

use super::{
    data_stack::{DataStack, DataStackError},
    evaluator::EvaluatorError,
    interpreter::Interpreter,
};

pub fn run_tests<P: Platform>(
    interpreter: &mut Interpreter<P>,
    platform_context: P::Context<'_>,
) -> Result<(), TestError<P::Error>> {
    interpreter.run_tests(platform_context)
}

#[derive(Debug, thiserror::Error)]
pub enum TestError<T> {
    #[error(transparent)]
    Evaluator(#[from] EvaluatorError<T>),

    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error(
        "Data stack not empty after evaluating test definitions: {data_stack}"
    )]
    DataStackNotEmptyAfterScriptEvaluation { data_stack: DataStack },

    #[error("Expected test to return one `bool`; left on stack: {data_stack}")]
    DataStackNotEmptyAfterTestRun { data_stack: DataStack },
}
