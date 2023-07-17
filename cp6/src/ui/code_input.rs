use sycamore::{
    component,
    flow::Indexed,
    reactive::{create_memo, create_signal, Scope, Signal},
    rt::JsCast,
    view,
    view::View,
    web::Html,
    Prop,
};
use web_sys::{Event, KeyboardEvent};

use crate::cp;

#[component]
pub fn CodeInput<'r, G: Html>(cx: Scope<'r>, props: Props<'r>) -> View<G> {
    let input = create_signal(cx, String::new());
    let error = create_signal(cx, String::new());

    let detect_enter = move |event: Event| {
        if let Some(event) = event.dyn_ref::<KeyboardEvent>() {
            if event.key() == "Enter" {
                let code = input.get();
                input.modify().clear();

                error.modify().clear();
                if let Err(err) = props.test_runner.modify().run_code(&code) {
                    error.set(err.to_string());
                };
                props
                    .test_runner
                    .modify()
                    .run_tests(&mut props.test_reports.modify());
            }
        }
    };

    let error_lines = create_memo(cx, || {
        let error = error.get();
        error
            .lines()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
    });
    let hide_errors = error_lines.map(cx, |lines| lines.is_empty());

    view! { cx,
        div(class="flex flex-col") {
            input(
                bind:value=input,
                on:keyup=detect_enter,
                type="text",
                class="my-4 ring-1",
                placeholder="Type code, execute with Enter key",
                autofocus=true,
            )
            div(
                hidden=*hide_errors.get(),
                class="max-w-fit max-h-fit bg-red-300 rounded p-4"
            ) {
                Indexed(
                    iterable=error_lines,
                    view=|cx, line| view! { cx, p { (line) } },
                )
            }
        }
    }
}

#[derive(Prop)]
pub struct Props<'r> {
    test_runner: &'r Signal<cp::TestRunner>,
    test_reports: &'r Signal<cp::TestReports>,
}
