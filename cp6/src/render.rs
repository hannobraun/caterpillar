use sycamore::prelude::*;

use crate::cp;

pub fn render(test_reports: Vec<cp::TestReport>) {
    sycamore::render(|cx| {
        let test_reports = create_signal(cx, test_reports);

        view! { cx,
            ul {
                Indexed(
                    iterable=test_reports,
                    view=|cx, test_report| view! {cx,
                        li { (test_report.name) }
                    }
                )
            }
        }
    });
}
