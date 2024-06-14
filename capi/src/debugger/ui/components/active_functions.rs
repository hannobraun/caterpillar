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
        let process = process.get();

        let view = match ActiveFunctions::new(process.as_ref()) {
            ActiveFunctions::Functions { functions } => functions
                .into_iter()
                .map(|function| {
                    view! {
                        <Function
                            function=function
                            events=events.clone() />
                    }
                })
                .collect_view(),
            ActiveFunctions::Message { message } => {
                view! {
                    <p class="w-full h-full absolute inset-y-0 flex justify-center items-center">
                        {message}
                    </p>
                }
                .into_view()
            }
        };

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
