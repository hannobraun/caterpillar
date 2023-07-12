use sycamore::{
    component,
    prelude::Indexed,
    reactive::{ReadSignal, Scope},
    view,
    view::View,
    web::Html,
};

use crate::{cp, ui::test_report::TestReport};

#[component]
pub fn TestRunResult<'r, G: Html>(cx: Scope<'r>, props: Props<'r>) -> View<G> {
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

#[derive(sycamore::Prop)]
pub struct Props<'r> {
    test_reports: &'r ReadSignal<cp::TestReports>,
}
