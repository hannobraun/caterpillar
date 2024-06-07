use capi_runtime::{runtime::DataStack, Program};
use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};

use crate::ui::components::panel::Panel;

#[allow(unused_braces)] // working around a warning from the `view!` macro
#[component]
pub fn StackExplorer(program: ReadSignal<Option<Program>>) -> impl IntoView {
    let data_stack = move || {
        let program = program.get()?;

        let previous = program.previous_data_stack;
        let current = program.evaluator.data_stack().clone();

        let view = view! {
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
        };

        Some(view)
    };

    view! {
        {data_stack}
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
