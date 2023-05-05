mod cp;
mod functions;
mod test_report;
mod tests;

fn main() -> anyhow::Result<()> {
    let mut functions = functions::define()?;
    let test_reports = tests::run(&mut functions)?;
    test_report::print(&test_reports);

    Ok(())
}
