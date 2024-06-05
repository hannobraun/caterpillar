use crate::{
    platform::Platform, repr::eval::value, runtime::call_stack::StackFrame,
};

use super::{
    data_stack::{DataStack, DataStackError},
    evaluator::EvaluatorError,
    interpreter::Interpreter,
};

pub fn run_tests<P: Platform>(
    interpreter: &mut Interpreter<P>,
    platform_context: P::Context<'_>,
) -> Result<TestReport<P::Error>, TestError<P::Error>> {
    let mut platform_context = platform_context;
    while !interpreter.step(&mut platform_context)?.finished() {}

    let tests = interpreter
        .evaluator()
        .global_namespace
        .global_module()
        .tests()
        .cloned()
        .collect::<Vec<_>>();

    if !interpreter.evaluator().data_stack.is_empty() {
        // This happens easily, if you do most of the work of defining a test,
        // but then forgot to actually write `test` at the end. Without this
        // error, it would result in dead code that's never actually run.
        return Err(TestError::DataStackNotEmptyAfterScriptEvaluation {
            data_stack: interpreter.evaluator().data_stack.clone(),
        });
    }

    let mut test_report = TestReport { inner: Vec::new() };

    for function in tests {
        // If all is well, both the call and data stacks should be empty at this
        // point. A failing test might have left traces on either though, and we
        // need to address this before running the next one.
        interpreter.evaluator().call_stack.clear();
        interpreter.evaluator().data_stack.clear();

        interpreter
            .evaluator()
            .call_stack
            .push(StackFrame::Fragment {
                fragment_id: function.body.start,
            });

        let result = run_single_test(interpreter, &mut platform_context);
        let report = SingleTestReport {
            test_name: function.name,
            result,
        };

        test_report.inner.push(report);
    }

    Ok(test_report)
}

fn run_single_test<P: Platform>(
    interpreter: &mut Interpreter<P>,
    platform_context: &mut P::Context<'_>,
) -> Result<(), SingleTestError<P::Error>> {
    while !interpreter.step(platform_context)?.finished() {}

    let result = interpreter
        .evaluator()
        .data_stack
        .pop_specific::<value::Bool>();

    let result = match result {
        Ok((result, _)) => result,
        // The error handling here is not great. The top return value not being
        // `bool` might be part of a larger problem, and we're not providing the
        // best information by not returning the full stack in that case.
        //
        // Problem is, by the time `pop_specific` failed, we don't have the full
        // stack any more. If we had an easy way to non-destructively check the
        // top value on the stack, then we could just return a single "expected
        // stack to return only `bool`; here's what it returned instead" error.
        Err(err) => return Err(SingleTestError::TestDidNotReturnBool(err)),
    };

    if !interpreter.evaluator().data_stack.is_empty() {
        return Err(SingleTestError::DataStackNotEmptyAfterTestRun {
            data_stack: interpreter.evaluator().data_stack.clone(),
        });
    }

    if !result.0 {
        return Err(SingleTestError::TestReturnedFalse);
    }

    Ok(())
}

#[must_use]
pub struct TestReport<E> {
    pub inner: Vec<SingleTestReport<E>>,
}

pub struct SingleTestReport<E> {
    pub test_name: String,
    pub result: Result<(), SingleTestError<E>>,
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
}

#[derive(Debug, thiserror::Error)]
pub enum SingleTestError<E> {
    #[error(transparent)]
    Evaluator(#[from] EvaluatorError<E>),

    #[error("Test did not return `bool`")]
    TestDidNotReturnBool(DataStackError),

    #[error("Expected test to return one `bool`; left on stack: {data_stack}")]
    DataStackNotEmptyAfterTestRun { data_stack: DataStack },

    #[error("Test returned `false`")]
    TestReturnedFalse,
}
