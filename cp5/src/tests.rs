use std::collections::BTreeMap;

use crate::{
    cp,
    test_report::{self, TestReport},
};

pub fn run() -> Vec<TestReport> {
    let mut tests = BTreeMap::new();

    tests.insert(("bool", "true"), "true");

    let mut results = Vec::new();

    for ((module, name), code) in tests {
        let result = match cp::execute(code) {
            true => Ok(()),
            false => Err(test_report::Error::TestFailed),
        };

        results.push(TestReport {
            module: module.into(),
            name: name.into(),
            result,
        })
    }

    results
}
