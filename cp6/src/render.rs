mod pass_fail;
mod test_report;

use sycamore::prelude::*;

use crate::{cp, render::test_report::TestReport};

pub fn render(test_reports: Vec<cp::TestReport>) {
    sycamore::render(|cx| {
        let test_reports = create_signal(cx, test_reports);

        view! { cx,
            TestReports(test_reports=test_reports)
        }
    });
}

#[component]
fn TestReports<'r, G: Html>(
    cx: Scope<'r>,
    props: TestReportsProps<'r>,
) -> View<G> {
    view! { cx,
        ul {
            Indexed(
                iterable=props.test_reports,
                view=|cx, test_report| view! { cx,
                    li {
                        TestReport(test_report=test_report)
                    }
                }
            )
        }
    }
}

#[derive(Prop)]
struct TestReportsProps<'r> {
    test_reports: &'r ReadSignal<Vec<cp::TestReport>>,
}
