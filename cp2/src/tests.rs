use std::collections::BTreeMap;

use crate::cp;

pub struct TestResult {
    pub name: &'static str,
    pub pass: bool,
}

pub fn run() -> Vec<TestResult> {
    let mut tests = BTreeMap::new();

    tests.insert("true", "true");

    let mut results = Vec::new();

    for (name, code) in tests {
        let mut data_stack = Vec::new();
        cp::evaluate(code, &mut data_stack);
        let pass = data_stack.pop().unwrap_or(false) && data_stack.is_empty();

        results.push(TestResult { name, pass });
    }

    results.sort_by_key(|result| result.pass);
    results.reverse();

    results
}
