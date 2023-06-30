use std::io::stdin;

use crate::{cp, test_report, tests};

pub fn run(
    functions: &mut cp::Functions,
    tests: &mut cp::Functions,
) -> anyhow::Result<()> {
    let mut data_stack = cp::DataStack::new();
    let mut bindings = cp::Bindings::new();

    for input in stdin().lines() {
        let input = input?;

        cp::execute(&input, &mut data_stack, &mut bindings, functions, tests)?;

        println!("{data_stack}");

        let test_reports = tests::run(functions, tests)?;
        test_report::print(&test_reports);
    }

    Ok(())
}
