use std::collections::BTreeMap;

use crate::cp;

pub struct TestReport {
    pub name: &'static str,
    pub result: Result<(), Error>,
}

pub fn run() -> Vec<TestReport> {
    let mut tests = BTreeMap::new();

    tests.insert("true", "true");
    tests.insert("false not", "false not");
    tests.insert("drop", "true false drop");

    let mut results = Vec::new();

    for (name, code) in tests {
        let mut data_stack = cp::DataStack::new();
        let tokens = cp::tokenize(code);
        let expressions = cp::parse(tokens);
        let result = cp::evaluate(expressions, &mut data_stack)
            .map_err(|err| Error::Evaluator(err))
            .and_then(|()| match data_stack.pop() {
                Ok(true) => Ok(()),
                _ => Err(Error::TestFailed),
            })
            .and_then(|()| {
                if data_stack.is_empty() {
                    Ok(())
                } else {
                    Err(Error::TestReturnedTooMuch)
                }
            });

        results.push(TestReport { name, result });
    }

    results.sort_by_key(|report| report.result.is_ok());
    results.reverse();

    results
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Evaluator(cp::EvaluatorError),

    #[error("Test did not return `true`")]
    TestFailed,

    #[error("Test returned too many values")]
    TestReturnedTooMuch,
}
