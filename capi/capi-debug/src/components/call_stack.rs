use capi_runtime::Program;
use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};

use crate::components::panel::Panel;

#[component]
pub fn CallStack(program: ReadSignal<Option<Program>>) -> impl IntoView {
    let addresses = move || {
        let program = program.get()?;

        let view = program
            .evaluator
            .call_stack
            .into_iter()
            .filter_map(|address| {
                let location =
                    program.source_map.address_to_location(&address)?;

                Some(view! {
                    <li>{format!("{location:?}")}</li>
                })
            })
            .collect_view();

        Some(view)
    };

    view! {
        <Panel class="">
            <div>
                <h2>"Call stack:"</h2>
                <ol>
                    {addresses}
                </ol>
            </div>
        </Panel>
    }
}
