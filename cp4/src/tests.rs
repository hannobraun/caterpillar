use std::collections::BTreeMap;

use crate::{cp, test_report::TestReport};

pub async fn run() -> anyhow::Result<Vec<TestReport>> {
    let mut tests = BTreeMap::new();
    let mut test_reports = Vec::new();

    tests.insert(("bool", "true"), "true");
    tests.insert(("bool", "false not"), "false not");
    tests.insert(("block", "eval"), "{ true } eval");
    tests.insert(("fn_", "fn"), "fn f { true } f");

    for ((module, name), code) in tests {
        let module = module.into();
        let name = name.into();
        let chars = cp::Chars::new(code);

        let mut data_stack = cp::DataStack::new();
        let mut functions = cp::Functions::new();
        let mut tests = cp::Functions::new();

        let result =
            cp::execute(chars, &mut data_stack, &mut functions, &mut tests)
                .await;

        test_reports.push(TestReport::new(module, name, result, data_stack));
    }

    Ok(test_reports)
}
