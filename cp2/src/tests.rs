use std::collections::BTreeMap;

use crate::cp;

pub struct TestResult {
    pub name: &'static str,
    pub pass: bool,
}

pub fn run() -> Vec<TestResult> {
    let mut tests = BTreeMap::new();

    tests.insert("bool", "true");

    let mut results = Vec::new();

    for (name, code) in tests {
        let pass = cp::evaluate(code);

        results.push(TestResult { name, pass });
    }

    results
}
