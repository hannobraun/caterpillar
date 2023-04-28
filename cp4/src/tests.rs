use crate::{cp, test_report::TestReport};

pub fn run() -> Vec<TestReport> {
    let (result, data_stack) = cp::execute("true");

    vec![TestReport::new(
        "test".into(),
        "test".into(),
        result,
        data_stack,
    )]
}
