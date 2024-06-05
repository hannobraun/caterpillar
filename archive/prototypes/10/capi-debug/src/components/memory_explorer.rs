use capi_runtime::{self, Program};
use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};

use crate::components::panel::Panel;

#[allow(unused_braces)] // working around a warning from the `view!` macro
#[component]
pub fn MemoryExplorer(program: ReadSignal<Option<Program>>) -> impl IntoView {
    let memory = move || {
        let program = program.get()?;

        let mut values = program.memory.inner.into_iter().peekable();
        let values = values.by_ref();

        let mut lines = Vec::new();

        while values.peek().is_some() {
            let line = values.take(16).collect::<Vec<_>>();
            lines.push(line);
        }

        let lines = lines
            .into_iter()
            .map(|line| {
                view! {
                    <Line line=line />
                }
            })
            .collect_view();

        let view = view! {
            <Panel class="">
                <p>"Memory:"</p>
                <ol>
                    {lines}
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
fn Line(line: Vec<capi_runtime::Value>) -> impl IntoView {
    let values = line
        .into_iter()
        .map(|value| {
            view! {
                <Value value=value />
            }
        })
        .collect_view();

    view! {
        <li>
            <ol>{values}</ol>
        </li>
    }
}

#[component]
fn Value(value: capi_runtime::Value) -> impl IntoView {
    view! {
        <li class="inline-block w-6 mr-2 text-right">{value.to_string()}</li>
    }
}
