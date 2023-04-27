use cp::DataStack;
use test_report::TestReport;

mod cp;
mod test_report;

fn main() -> anyhow::Result<()> {
    let test_reports = vec![TestReport::new(
        "test".into(),
        "test".into(),
        Ok(()),
        DataStack::new(),
    )];
    test_report::print(&test_reports);

    Ok(())
}
