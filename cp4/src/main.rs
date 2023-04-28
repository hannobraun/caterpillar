mod cp;
mod test_report;
mod tests;

fn main() -> anyhow::Result<()> {
    let test_reports = tests::run();
    test_report::print(&test_reports);

    Ok(())
}
