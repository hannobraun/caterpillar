use crate::{cp, test_report::TestReport};

pub fn run() -> Vec<TestReport> {
    let module = "test".into();
    let name = "test".into();

    let (result, data_stack) = cp::execute("true");

    vec![TestReport::new(module, name, result, data_stack)]
}
