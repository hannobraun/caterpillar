use test_report::TestReport;

mod cp;
mod test_report;

fn main() -> anyhow::Result<()> {
    let result = Ok(());
    let data_stack = cp::DataStack::new();
    let test_reports = vec![TestReport::new(
        "test".into(),
        "test".into(),
        result,
        data_stack,
    )];
    test_report::print(&test_reports);

    Ok(())
}
