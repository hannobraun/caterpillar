use ::std::io::stdin;

mod cp;
mod std;
mod test_report;
mod tests;

fn main() -> anyhow::Result<()> {
    let functions = std::define()?;
    let test_reports = tests::run(functions)?;
    test_report::print(&test_reports);

    for input in stdin().lines() {
        let input = input?;
        println!("{input}");
    }

    Ok(())
}
