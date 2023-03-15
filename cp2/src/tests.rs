use std::collections::BTreeMap;

pub struct TestResult {
    pub name: &'static str,
    pub pass: bool,
}

pub fn run() -> Vec<TestResult> {
    let mut tests = BTreeMap::new();

    tests.insert("bool", "true");

    let mut results = Vec::new();

    for (name, code) in tests {
        let pass = code == "true";

        results.push(TestResult { name, pass });
    }

    results
}
