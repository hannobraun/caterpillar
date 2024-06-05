use sycamore::{
    component,
    reactive::{Scope, Signal},
    view,
    view::View,
    web::Html,
    Prop,
};

use crate::{
    cp,
    ui::{code_input::CodeInput, function_list::FunctionList},
};

#[component]
pub fn CodeEditor<'r, G: Html>(cx: Scope<'r>, props: Props<'r>) -> View<G> {
    let functions = props.test_runner.map(cx, |test_runner| {
        (test_runner.functions().clone(), test_runner.tests().clone())
    });

    view! { cx,
        div(class="h-full flex flex-col p-4") {
            div(class="overflow-auto p-2") {
                FunctionList(functions=functions)
            }
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
