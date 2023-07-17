use sycamore::{
    component,
    prelude::Indexed,
    reactive::{Scope, Signal},
    view,
    view::View,
    web::Html,
    Prop,
};

use crate::{cp, ui::code_input::CodeInput};

#[component]
pub fn CodeEditor<'r, G: Html>(cx: Scope<'r>, props: Props<'r>) -> View<G> {
    let functions = props.test_runner.map(cx, |test_runner| {
        test_runner
            .functions()
            .iter()
            .map(|(_, function)| function)
            .cloned()
            .collect::<Vec<_>>()
    });

    view! { cx,
        div(class="p-4") {
            Indexed(
                iterable=functions,
                view=|cx, function| view! { cx, p { (function.name) } },
            )
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
