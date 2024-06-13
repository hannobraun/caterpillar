use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};

use crate::{
    program::Process,
    ui::{
        components::{function::Function, panel::Panel},
        EventsTx,
    },
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

                Some(view! {
                    <Function
                        program=process
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
