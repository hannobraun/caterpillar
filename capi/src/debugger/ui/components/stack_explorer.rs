use leptos::{component, view, CollectView, IntoView};

use crate::{
    debugger::ui::components::panel::Panel, process::Process,
    runtime::DataStack,
};

#[allow(unused_braces)] // working around a warning from the `view!` macro
#[component]
pub fn StackExplorer(process: Process) -> impl IntoView {
    let previous = process.previous_data_stack;
    let current = process.evaluator.data_stack().clone();

    view! {
        <Panel class="h-32">
            <div>
                <p>
                    "Previous data stack:"
                </p>
                <DataStack data_stack=previous />
            </div>
            <div>
                <p>
                    "Current data stack:"
                </p>
                <DataStack data_stack=current />
            </div>
        </Panel>
    }
}

#[component]
pub fn DataStack(data_stack: DataStack) -> impl IntoView {
    let values = data_stack
        .values()
        .map(|value| {
            view! {
                <li class="inline-block mr-2">{value.to_string()}</li>
            }
        })
        .collect_view();

    view! {
        <ol>
            {values}
        </ol>
    }
}
