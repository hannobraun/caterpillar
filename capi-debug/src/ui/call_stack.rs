use capi_runtime::Program;
use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};

use crate::{
    client::EventsTx,
    ui::{function::Function, panel::Panel},
};

#[component]
pub fn CallStack(
    program: ReadSignal<Option<Program>>,
    events: EventsTx,
) -> impl IntoView {
    let addresses = move || {
        let view = program
            .get()?
            .evaluator
            .call_stack()
            .iter()
            .filter_map(|runtime_location| {
                let syntax_location = program
                    .get()?
                    .source_map
                    .runtime_to_syntax(&runtime_location);
                let function = program
                    .get()?
                    .functions
                    .get_from_location(syntax_location)
                    .cloned()?;

                Some(view! {
                    <Function
                        program=program
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
