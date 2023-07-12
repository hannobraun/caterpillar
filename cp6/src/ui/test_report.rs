use sycamore::{component, reactive::Scope, view, view::View, web::Html};

use crate::{cp, ui::pass_fail::PassFail};

#[component]
pub fn TestReport<G: Html>(cx: Scope, props: Props) -> View<G> {
    view! { cx,
        PassFail(pass=props.test_report.result.is_ok())
        (props.test_report.module) " - " (props.test_report.name)
    }
}

#[derive(sycamore::Prop)]
pub struct Props {
    test_report: cp::TestReport,
}
