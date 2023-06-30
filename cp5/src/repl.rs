use std::io::stdin;

use crate::{
    cp::{self, AnalyzerEvent, FunctionBody},
    test_report, tests,
};

pub fn run(
    functions: &mut cp::Functions,
    tests: &mut cp::Functions,
) -> anyhow::Result<()> {
    let mut data_stack = cp::DataStack::new();
    let mut bindings = cp::Bindings::new();

    functions.clear_updated();

    for input in stdin().lines() {
        let input = input?;

        cp::execute(&input, &mut data_stack, &mut bindings, functions, tests)?;

        println!("{data_stack}");

        let mut updated = functions.clear_updated();
        let mut found_new_updated;

        loop {
            found_new_updated = false;

            for (name, function) in &*functions {
                if updated.contains(name) {
                    continue;
                }

                if let FunctionBody::UserDefined(analyzer_output) =
                    &function.body
                {
                    for event in analyzer_output.all_events_recursive() {
                        if let AnalyzerEvent::EvalFunction { name: called } =
                            event
                        {
                            if updated.contains(called) {
                                updated.insert(name.clone());
                                found_new_updated = true;
                            }
                        }
                    }
                }
            }

            if !found_new_updated {
                break;
            }
        }

        dbg!(updated);

        let test_reports = tests::run(functions, tests)?;
        test_report::print(&test_reports);
    }

    Ok(())
}
