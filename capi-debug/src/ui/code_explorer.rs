use capi_runtime::Program;
use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};

use crate::{client::EventsTx, ui::function::Function};

#[component]
pub fn CodeExplorer(
    program: ReadSignal<Option<Program>>,
    events: EventsTx,
) -> impl IntoView {
    let functions = move || {
        let view = program
            .get()?
            .functions
            .inner
            .into_iter()
            .map(|f| {
                view! {
                    <Function
                        program=program
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
