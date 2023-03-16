use std::collections::BTreeMap;

use crate::cp;

pub struct TestResult {
    pub name: &'static str,
    pub pass: bool,
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
        let pass = result.is_ok()
            && data_stack.pop().unwrap_or(false)
            && data_stack.is_empty();

        results.push(TestResult { name, pass });
    }

    results.sort_by_key(|result| result.pass);
    results.reverse();

    results
}
