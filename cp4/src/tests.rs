use std::collections::BTreeMap;

use crate::{cp, test_report::TestReport};

pub fn run() -> Vec<TestReport> {
    let mut tests = BTreeMap::new();
    let mut test_reports = Vec::new();

    tests.insert(("bool".into(), "true".into()), "true");

    for ((module, name), code) in tests {
        let (result, data_stack) = cp::execute(code);
        test_reports.push(TestReport::new(module, name, result, data_stack));
    }

    test_reports
}
