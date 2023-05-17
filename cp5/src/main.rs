mod cp;
mod test_report;

fn main() -> anyhow::Result<()> {
    let test_reports = Vec::new();
    test_report::print(&test_reports);

    Ok(())
}
