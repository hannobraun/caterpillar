use ::std::io::stdin;

mod cp;
mod std;
mod test_report;
mod tests;

fn main() -> anyhow::Result<()> {
    let mut functions = std::define()?;
    let tests = tests::define(&mut functions)?;
    let test_reports = tests::run(functions, tests)?;
    test_report::print(&test_reports);

    for input in stdin().lines() {
        let input = input?;
        println!("{input}");
    }

    Ok(())
}
