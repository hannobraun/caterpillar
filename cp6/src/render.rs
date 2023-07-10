mod pass_fail;
mod test_report;
mod test_run_result;

use sycamore::prelude::*;

use crate::{cp, render::test_run_result::TestRunResult};

pub fn render(test_reports: Vec<cp::TestReport>) {
    sycamore::render(|cx| {
        let test_reports = create_signal(cx, test_reports);

        view! { cx,
            input(type="text", class="m-4 ring-1")
            TestRunResult(test_reports=test_reports)
        }
    });
}
