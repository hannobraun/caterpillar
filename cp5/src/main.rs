mod cp;
mod intrinsics;
mod repl;
mod std;
mod test_report;
mod tests;

fn main() -> anyhow::Result<()> {
    let mut functions = cp::Functions::new();
    intrinsics::define(&mut functions);
    std::define(&mut functions)?;

    let mut tests = tests::define(&mut functions)?;
    let test_reports = tests::run(&functions, &tests)?;
    test_report::print(&test_reports);

    repl::run(&mut functions, &mut tests)?;

    Ok(())
}
