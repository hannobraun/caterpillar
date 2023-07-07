use sycamore::prelude::*;

use crate::cp;

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

#[component]
fn TestReport<G: Html>(cx: Scope, props: TestReportProps) -> View<G> {
    view! { cx,
        PassFail(pass=props.test_report.result.is_ok())
        (props.test_report.module) " - " (props.test_report.name)
    }
}

#[derive(Prop)]
struct TestReportProps {
    test_report: cp::TestReport,
}

#[component]
fn PassFail<G: Html>(cx: Scope, props: PassFailProps) -> View<G> {
    let class = {
        let color = if props.pass {
            "text-green-500"
        } else {
            "text-red-500"
        };

        format!("font-bold mx-2 {color}")
    };
    let text = if props.pass { "PASS" } else { "FAIL" };

    view! { cx,
        span(class=class) { (text) }
    }
}

#[derive(Prop)]
struct PassFailProps {
    pass: bool,
}
