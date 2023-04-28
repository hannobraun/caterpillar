use crate::{cp, test_report::TestReport};

pub fn run() -> Vec<TestReport> {
    let mut test_reports = Vec::new();

    let module = "bool".into();
    let name = "true".into();
    let code = "true";

    let (result, data_stack) = cp::execute(code);
    test_reports.push(TestReport::new(module, name, result, data_stack));

    test_reports
}
