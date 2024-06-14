use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};

use crate::{
    debugger::{
        model::Function,
        ui::{
            components::{function::Function, panel::Panel},
            EventsTx,
        },
    },
    process::Process,
};

#[component]
pub fn CallStack(
    process: ReadSignal<Option<Process>>,
    events: EventsTx,
) -> impl IntoView {
    let addresses = move || {
        let view = process
            .get()?
            .evaluator
            .stack()
            .iter()
            .filter_map(|runtime_location| {
                let syntax_location = process
                    .get()?
                    .source_map
                    .runtime_to_syntax(&runtime_location);
                let function = process
                    .get()?
                    .functions
                    .get_from_location(syntax_location)
                    .cloned()?;
                let function = Function::new(function, &process.get()?);

                Some(view! {
                    <Function
                        function=function
                        events=events.clone() />
                })
            })
            .collect_view();

        Some(view)
    };

    view! {
        <Panel class="h-64">
            <div>
                <h2>"Call stack:"</h2>
                <ol>
                    {addresses}
                </ol>
            </div>
        </Panel>
    }
}
