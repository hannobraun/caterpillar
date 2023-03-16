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

    let mut results = Vec::new();

    for (name, code) in tests {
        let mut data_stack = Vec::new();
        let tokens = cp::tokenize(code);
        let result = cp::evaluate(tokens, &mut data_stack)
            .map_err(|err| Error::Evaluator(err))
            .and_then(|()| match data_stack.pop() {
                Some(true) => Ok(()),
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

pub enum Error {
    Evaluator(cp::EvaluatorError),
    TestFailed,
    TestReturnedTooMuch,
}
