use capi_runtime::Program;
use leptos::{component, view, IntoView, ReadSignal, SignalGet};

#[allow(unused_braces)] // working around a warning from the `view!` macro
#[component]
pub fn StackExplorer(program: ReadSignal<Option<Program>>) -> impl IntoView {
    let data_stack = move || {
        let program = program.get()?;

        let view = view! {
            <div>
                <p>
                    "Previous data stack: "
                    {format!("{:?}", program.previous_data_stack)}
                </p>
                <p>
                    "Current data stack: "
                    {format!("{:?}", program.evaluator.data_stack)}
                </p>
            </div>
        };

        Some(view)
    };

    view! {
        {data_stack}
    }
}
