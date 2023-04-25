mod cp;
mod test_report;
mod tests;

fn main() -> anyhow::Result<()> {
    let mut functions = cp::Functions::new();
    let test_reports = tests::run(&mut functions)?;
    test_report::print(&test_reports);

    Ok(())
}
