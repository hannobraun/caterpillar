use std::collections::BTreeMap;

use crate::{
    cp,
    test_report::{self, TestReport},
};

pub fn run() -> Result<Vec<TestReport>, test_report::Error> {
    let mut tests = BTreeMap::new();

    tests.insert(("bool", "true"), "true");

    let mut results = Vec::new();

    for ((module, name), code) in tests {
        let mut data_stack = cp::DataStack::new();

        cp::execute(code, &mut data_stack);

        let result = if data_stack.pop()? {
            Ok(())
        } else {
            Err(test_report::Error::TestFailed)
        };

        results.push(TestReport {
            module: module.into(),
            name: name.into(),
            result,
        })
    }

    Ok(results)
}
