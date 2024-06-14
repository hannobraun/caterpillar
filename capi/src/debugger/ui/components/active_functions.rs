use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};

use crate::{
    debugger::{
        model::ActiveFunctions,
        ui::{
            components::{function::Function, panel::Panel},
            EventsTx,
        },
    },
    process::Process,
};

#[component]
pub fn ActiveFunctions(
    process: ReadSignal<Option<Process>>,
    events: EventsTx,
) -> impl IntoView {
    let functions = move || {
        let process = process.get()?;

        let active_functions = ActiveFunctions::new(&process);

        let view = active_functions
            .functions
            .into_iter()
            .map(|function| {
                view! {
                    <Function
                        function=function
                        events=events.clone() />
                }
            })
            .collect_view();

        Some(view)
    };

    view! {
        <Panel class="h-64">
            <div>
                <h2>"Call stack:"</h2>
                <ol>
                    {functions}
                </ol>
            </div>
        </Panel>
    }
}
