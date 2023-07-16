mod code_input;
mod pass_fail;
mod test_report;
mod test_run_result;

use sycamore::prelude::*;

use crate::{
    cp,
    ui::{code_input::CodeInput, test_run_result::TestRunResult},
};

pub fn render(mut test_runner: cp::TestRunner) {
    let mut test_reports = cp::TestReports::new();
    test_runner.run_tests(&mut test_reports);

    sycamore::render(|cx| {
        let test_reports = create_signal(cx, test_reports);

        view! { cx,
            div {
                CodeInput(test_runner=test_runner, test_reports=test_reports)
                TestRunResult(test_reports=test_reports)
            }
        }
    });
}
