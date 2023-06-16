use ::std::io::stdin;

mod cp;
mod std;
mod test_report;
mod tests;

fn main() -> anyhow::Result<()> {
    let mut functions = std::define()?;
    let mut tests = tests::define(&mut functions)?;
    let test_reports = tests::run(&mut functions, &tests)?;
    test_report::print(&test_reports);

    let mut data_stack = cp::DataStack::new();
    let mut bindings = cp::Bindings::new();

    for input in stdin().lines() {
        let input = input?;

        cp::execute(
            &input,
            &mut data_stack,
            &mut bindings,
            &mut functions,
            &mut tests,
        )?;

        println!("{data_stack}");
    }

    Ok(())
}
