use cp::DataStack;
use test_report::TestReport;

mod cp;
mod test_report;

fn main() -> anyhow::Result<()> {
    let data_stack = DataStack::new();
    let test_reports = vec![TestReport::new(
        "test".into(),
        "test".into(),
        Ok(()),
        data_stack,
    )];
    test_report::print(&test_reports);

    Ok(())
}
