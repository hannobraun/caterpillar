use leptos::{component, view, CollectView, IntoView};

use crate::debugger::{
    model::ActiveFunctions,
    ui::{
        components::{function::Function, panel::Panel},
        EventsTx,
    },
};

#[component]
pub fn ActiveFunctions(
    active_functions: ActiveFunctions,
    events: EventsTx,
) -> impl IntoView {
    let functions = match active_functions {
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
