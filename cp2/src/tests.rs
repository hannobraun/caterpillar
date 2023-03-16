use std::collections::BTreeMap;

use crate::cp;

pub struct TestResult {
    pub name: &'static str,
    pub pass: Result<(), ()>,
}

pub fn run() -> Vec<TestResult> {
    let mut tests = BTreeMap::new();

    tests.insert("true", "true");
    tests.insert("false not", "false not");

    let mut results = Vec::new();

    for (name, code) in tests {
        let mut data_stack = Vec::new();
        let tokens = cp::tokenize(code);
        let result = cp::evaluate(tokens, &mut data_stack);
        let pass = result
            .map_err(|_| ())
            .and_then(|()| match data_stack.pop() {
                Some(true) => Ok(()),
                _ => Err(()),
            })
            .and_then(|()| {
                if data_stack.is_empty() {
                    Ok(())
                } else {
                    Err(())
                }
            });

        results.push(TestResult { name, pass });
    }

    results.sort_by_key(|result| result.pass.is_ok());
    results.reverse();

    results
}
