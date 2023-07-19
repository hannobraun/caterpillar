use std::collections::{BTreeMap, BTreeSet};

use js_sys::Date;

use crate::cp;

use super::{AnalyzerEvent, FunctionBody};

#[derive(Clone)]
pub struct TestRunner {
    data_stack: cp::DataStack,
    bindings: cp::Bindings,
    functions: cp::Functions,
    tests: cp::Functions,
}

impl TestRunner {
    pub fn new() -> anyhow::Result<Self> {
        let data_stack = cp::DataStack::new();
        let bindings = cp::Bindings::new();
        let (functions, tests) = cp::define_code()?;

        Ok(Self {
            data_stack,
            bindings,
            functions,
            tests,
        })
    }

    pub fn functions(&self) -> &cp::Functions {
        &self.functions
    }

    pub fn tests(&self) -> &cp::Functions {
        &self.tests
    }

    pub fn run_code(&mut self, code: &str) -> anyhow::Result<()> {
        cp::execute(
            code,
            &mut self.data_stack,
            &mut self.bindings,
            &mut self.functions,
            &mut self.tests,
        )?;

        Ok(())
    }

    pub fn run_tests(&mut self, test_reports: &mut TestReports) {
        let mut updated = self.functions.clear_updated();
        let mut found_new_updated;

        loop {
            found_new_updated = false;

            for (name, function) in &self.functions {
                if updated.contains(name) {
                    continue;
                }

                if let FunctionBody::UserDefined(analyzer_output) =
                    &function.body
                {
                    for event in analyzer_output.all_events_recursive() {
                        if let AnalyzerEvent::EvalFunction { name: called } =
                            event
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

            for (_, function) in &self.tests {
                if tests_to_run.contains(&function.name) {
                    continue;
                }

                if let FunctionBody::UserDefined(analyzer_output) =
                    &function.body
                {
                    for event in analyzer_output.all_events_recursive() {
                        if let AnalyzerEvent::EvalFunction { name: called } =
                            event
                        {
                            if updated.contains(called) {
                                tests_to_run.insert(function.name.clone());
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

        for name in tests_to_run {
            let function = self.tests.get(&name);

            let mut data_stack = cp::DataStack::new();
            let mut bindings = cp::Bindings::new();

            let mut evaluator = cp::Evaluator {
                data_stack: &mut data_stack,
                bindings: &mut bindings,
                functions: &self.functions,
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

            let module = function.module.clone();
            let name = name.clone();

            test_reports.inner.insert(
                (module.clone(), name.clone()),
                SingleTestReport {
                    module,
                    name,
                    result,
                    timestamp: Date::now(),
                },
            );
        }
    }
}

#[derive(Clone)]
pub struct TestReports {
    inner: BTreeMap<(String, String), SingleTestReport>,
}

impl TestReports {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn reports(&self) -> Vec<SingleTestReport> {
        self.inner.values().cloned().collect()
    }
}

#[derive(Clone, PartialEq)]
pub struct SingleTestReport {
    pub module: String,
    pub name: String,
    pub result: Result<(), Error>,
    pub timestamp: f64,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
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
