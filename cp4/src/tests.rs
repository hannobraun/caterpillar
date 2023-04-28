use crate::{cp, test_report::TestReport};

pub fn run() -> Vec<TestReport> {
    let result = Ok(());
    let data_stack = cp::DataStack::new();

    vec![TestReport::new(
        "test".into(),
        "test".into(),
        result,
        data_stack,
    )]
}
