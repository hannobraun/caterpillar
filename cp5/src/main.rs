mod cp;
mod test_report;
mod tests;

fn main() -> anyhow::Result<()> {
    let functions = cp::Functions::new();
    let test_reports = tests::run(functions)?;
    test_report::print(&test_reports);

    Ok(())
}
