use sycamore::{
    component,
    reactive::{Scope, Signal},
    view,
    view::View,
    web::Html,
    Prop,
};

use crate::{cp, ui::code_input::CodeInput};

#[component]
pub fn CodeEditor<'r, G: Html>(cx: Scope<'r>, props: Props<'r>) -> View<G> {
    view! { cx,
        div {
            CodeInput(
                test_runner=props.test_runner,
                test_reports=props.test_reports
            )
        }
    }
}

#[derive(Prop)]
pub struct Props<'r> {
    test_runner: &'r Signal<cp::TestRunner>,
    test_reports: &'r Signal<cp::TestReports>,
}
