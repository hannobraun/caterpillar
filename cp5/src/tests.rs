use std::collections::BTreeMap;

use crate::{
    cp,
    test_report::{Error, TestReport},
};

pub fn run() -> anyhow::Result<Vec<TestReport>> {
    let mut tests = BTreeMap::new();

    tests.insert(("bool", "true"), "true");
    tests.insert(("bool", "false not"), "false not");

    let mut results = Vec::new();

    for ((module, name), code) in tests {
        let mut data_stack = cp::DataStack::new();

        let result = cp::execute(code, &mut data_stack)
            .map_err(Error::Evaluator)
            .and_then(|()| {
                if data_stack.pop()? {
                    Ok(())
                } else {
                    Err(Error::TestFailed)
                }
            });

        results.push(TestReport {
            module: module.into(),
            name: name.into(),
            result,
        })
    }

    Ok(results)
}
