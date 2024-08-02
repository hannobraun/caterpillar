use capi_process::{Operands, Value};
use leptos::{component, view, CollectView, IntoView};

use crate::ui::components::panel::Panel;

#[allow(unused_braces)] // working around a warning from the `view!` macro
#[component]
pub fn StackExplorer(current: Operands) -> impl IntoView {
    let current = current.values().collect();
    view! {
        <Panel class="h-32">
            <div>
                <p>
                    "Current data stack:"
                </p>
                <Operands operands=current />
            </div>
        </Panel>
    }
}

#[component]
pub fn Operands(operands: Vec<Value>) -> impl IntoView {
    let values = operands
        .into_iter()
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
