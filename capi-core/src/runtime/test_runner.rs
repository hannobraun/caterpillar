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
) -> Result<TestReport, TestError<P::Error>> {
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
        // We don't need to worry about any call stack contents from the initial
        // module evaluation, or the evaluation of the previous test,
        // interfering with the evaluation of the next test. When evaluation is
        // finished then, by definition, the call stack is empty.
        //
        // (We have to clear the data stack before the next test run though.)
        interpreter
            .evaluator()
            .call_stack
            .push(StackFrame::Fragment {
                fragment_id: function.body.start,
            });
        interpreter.evaluator().data_stack.clear();

        while !interpreter.step(&mut platform_context)?.finished() {}

        let result = {
            let (result, _) = interpreter
                .evaluator()
                .data_stack
                .pop_specific::<value::Bool>()?;

            if !interpreter.evaluator().data_stack.is_empty() {
                return Err(TestError::DataStackNotEmptyAfterTestRun {
                    data_stack: interpreter.evaluator().data_stack.clone(),
                });
            }

            if result.0 {
                Ok(())
            } else {
                Err(())
            }
        };

        test_report.inner.push(SingleTestReport {
            test_name: function.name.value,
            result,
        });
    }

    Ok(test_report)
}

#[must_use]
pub struct TestReport {
    pub inner: Vec<SingleTestReport>,
}

pub struct SingleTestReport {
    pub test_name: String,
    pub result: Result<(), ()>,
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
