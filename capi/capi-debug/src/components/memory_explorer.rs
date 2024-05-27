use capi_runtime::{self, Program};
use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};

use crate::components::panel::Panel;

#[allow(unused_braces)] // working around a warning from the `view!` macro
#[component]
pub fn MemoryExplorer(program: ReadSignal<Option<Program>>) -> impl IntoView {
    let memory = move || {
        let program = program.get()?;

        let values = program
            .memory
            .inner
            .into_iter()
            .map(|value| {
                view! {
                    <Value value=value />
                }
            })
            .collect_view();

        let view = view! {
            <Panel class="">
                <p>"Memory:"</p>
                <ol>
                    {values}
                </ol>
            </Panel>
        };

        Some(view)
    };

    view! {
        {memory}
    }
}

#[component]
fn Value(value: capi_runtime::Value) -> impl IntoView {
    view! {
        <li class="inline-block w-6 mr-2 text-right">{value.to_string()}</li>
    }
}
