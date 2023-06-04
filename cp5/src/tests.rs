use std::collections::BTreeMap;

use crate::{
    cp,
    test_report::{Error, TestReport},
};

pub fn run() -> anyhow::Result<Vec<TestReport>> {
    let mut tests = BTreeMap::new();

    tests.insert(("bool", "true"), "true");
    tests.insert(("bool", "false not"), "false not");
    tests.insert(("block", "eval"), "{ true } eval");
    tests.insert(("fn_", "fn"), "fn f { true } f");
    tests.insert(("string", "="), r#""a" "a" ="#);

    let mut results = Vec::new();

    for ((module, name), code) in tests {
        let mut data_stack = cp::DataStack::new();
        let mut functions = cp::Functions::new();
        let mut tests = cp::Functions::new();

        let end_result =
            cp::execute(code, &mut data_stack, &mut functions, &mut tests);

        let result = end_result
            .map_err(Error::Language)
            .and_then(|()| {
                let test_passed = data_stack.pop_bool()?;
                if test_passed {
                    Ok(())
                } else {
                    Err(Error::TestFailed)
                }
            })
            .and_then(|()| {
                if data_stack.is_empty() {
                    Ok(())
                } else {
                    Err(Error::TestReturnedTooMuch)
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
