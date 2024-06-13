use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};

use crate::{
    program::Process,
    ui::{components::function::Function, EventsTx},
};

#[component]
pub fn CodeExplorer(
    process: ReadSignal<Option<Process>>,
    events: EventsTx,
) -> impl IntoView {
    let functions = move || {
        let view = process
            .get()?
            .functions
            .inner
            .into_iter()
            .map(|f| {
                view! {
                    <Function
                        process=process
                        function=f
                        events=events.clone() />
                }
            })
            .collect_view();

        Some(view)
    };

    view! {
        <div>{functions}</div>
    }
}
