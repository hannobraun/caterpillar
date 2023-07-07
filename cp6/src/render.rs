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
                        (
                            if test_report.result.is_ok() {
                                view! { cx,
                                    span(class="font-bold text-green-500 mx-2")
                                    {
                                        "PASS"
                                    }
                                }
                            }
                            else {
                                view! { cx,
                                    span(class="font-bold text-red-500 mx-2") {
                                        "FAIL"
                                    }
                                }
                            }
                        )
                        (test_report.module) " - " (test_report.name)
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
