use std::collections::BTreeMap;

use crate::{
    cp,
    test_report::{Error, TestReport},
};

pub fn run() -> anyhow::Result<Vec<TestReport>> {
    let mut tests = BTreeMap::new();

    tests.insert(("bool", "true"), "true");

    let mut results = Vec::new();

    for ((module, name), code) in tests {
        let mut data_stack = cp::DataStack::new();

        cp::execute(code, &mut data_stack);

        let result = if data_stack.pop()? {
            Ok(())
        } else {
            Err(Error::TestFailed)
        };

        results.push(TestReport {
            module: module.into(),
            name: name.into(),
            result,
        })
    }

    Ok(results)
}
