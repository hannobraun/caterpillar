mod pass_fail;
mod test_report;
mod test_run_result;

use sycamore::{prelude::*, rt::JsCast};
use web_sys::{Event, KeyboardEvent};

use crate::{cp, ui::test_run_result::TestRunResult};

pub fn render(mut test_runner: cp::TestRunner) {
    let test_reports = test_runner.run_tests();

    sycamore::render(|cx| {
        let input = create_signal(cx, String::new());
        let test_reports = create_signal(cx, test_reports);

        let detect_enter = move |event: Event| {
            if let Some(event) = event.dyn_ref::<KeyboardEvent>() {
                if event.key() == "Enter" {
                    let code = input.get();
                    input.modify().clear();

                    if let Err(err) = test_runner.run_code(&code) {
                        log::error!("{err}");
                    }
                    let reports = test_runner.run_tests();

                    test_reports.set(reports);
                }
            }
        };

        view! { cx,
            input(
                bind:value=input,
                on:keyup=detect_enter,
                type="text",
                class="m-4 ring-1",
                autofocus=true,
            )
            TestRunResult(test_reports=test_reports)
        }
    });
}
