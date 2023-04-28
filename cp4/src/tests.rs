use crate::{cp, test_report::TestReport};

pub fn run() -> Vec<TestReport> {
    let module = "bool".into();
    let name = "true".into();
    let code = "true";

    let (result, data_stack) = cp::execute(code);

    vec![TestReport::new(module, name, result, data_stack)]
}
