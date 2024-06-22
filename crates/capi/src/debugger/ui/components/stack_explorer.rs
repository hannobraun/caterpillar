use leptos::{component, view, CollectView, IntoView};

use crate::{debugger::ui::components::panel::Panel, runtime::Operands};

#[allow(unused_braces)] // working around a warning from the `view!` macro
#[component]
pub fn StackExplorer(current: Operands) -> impl IntoView {
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
pub fn Operands(operands: Operands) -> impl IntoView {
    let values = operands
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
