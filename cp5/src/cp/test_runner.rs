use std::collections::BTreeSet;

use crate::cp;

use super::{AnalyzerEvent, FunctionBody};

pub fn run_tests(
    functions: &mut cp::Functions,
    tests: &cp::Functions,
) -> anyhow::Result<Vec<TestReport>> {
    let mut updated = functions.clear_updated();
    let mut found_new_updated;

    loop {
        found_new_updated = false;

        for (name, function) in &*functions {
            if updated.contains(name) {
                continue;
            }

            if let FunctionBody::UserDefined(analyzer_output) = &function.body {
                for event in analyzer_output.all_events_recursive() {
                    if let AnalyzerEvent::EvalFunction { name: called } = event
                    {
                        if updated.contains(called) {
                            updated.insert(name.clone());
                            found_new_updated = true;
                        }
                    }
                }
            }
        }

        if !found_new_updated {
            break;
        }
    }

    let mut tests_to_run = BTreeSet::new();
    let mut found_new_tests_to_run;

    loop {
        found_new_tests_to_run = false;

        for (name, function) in tests {
            if tests_to_run.contains(name) {
                continue;
            }

            if let FunctionBody::UserDefined(analyzer_output) = &function.body {
                for event in analyzer_output.all_events_recursive() {
                    if let AnalyzerEvent::EvalFunction { name: called } = event
                    {
                        if updated.contains(called) {
                            tests_to_run.insert(name.clone());
                            found_new_tests_to_run = true;
                        }
                    }
                }
            }
        }

        if !found_new_tests_to_run {
            break;
        }
    }

    let mut results = Vec::new();

    for name in tests_to_run {
        let function = tests.get(&name);

        let mut data_stack = cp::DataStack::new();
        let mut bindings = cp::Bindings::new();
        let tests = cp::Functions::new();

        let mut evaluator = cp::Evaluator {
            data_stack: &mut data_stack,
            bindings: &mut bindings,
            functions,
            tests: &tests,
        };

        let result = evaluator.evaluate_function(function);

        let result = result
            .map_err(Error::Evaluator)
            .and_then(|()| {
                let test_passed = data_stack.pop_bool()?;
                if test_passed {
                    Ok(())
                } else {
                    Err(Error::TestFailed)
                }
            })
            .and_then(|()| {
                if data_stack.is_empty() {
                    Ok(())
                } else {
                    Err(Error::TestReturnedTooMuch)
                }
            });

        results.push(TestReport {
            module: function.module.clone(),
            name: name.clone(),
            result,
        })
    }

    Ok(results)
}

pub struct TestReport {
    pub module: String,
    pub name: String,
    pub result: Result<(), Error>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Evaluator(cp::EvaluatorError),

    #[error(transparent)]
    ReturnValue(#[from] cp::DataStackError),

    #[error("Test did not return `true`")]
    TestFailed,

    #[error("Test returned too many values")]
    TestReturnedTooMuch,
}
