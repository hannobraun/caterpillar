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
    let functions = move || {
        let process = process.get()?;

        let functions = process
            .evaluator
            .stack()
            .iter()
            .filter_map(|runtime_location| {
                let syntax_location =
                    process.source_map.runtime_to_syntax(&runtime_location);
                let function = process
                    .functions
                    .get_from_location(syntax_location)
                    .cloned()?;

                Some(Function::new(function, &process))
            })
            .collect::<Vec<_>>();

        let view = functions
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
