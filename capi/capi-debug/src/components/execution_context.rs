use capi_runtime::Program;
use leptos::{component, view, IntoView, ReadSignal};

use crate::{
    client::EventsTx, components::function::Function, state::ExecutionContext,
};

use super::panel::Panel;

#[component]
pub fn ExecutionContext(
    program: ReadSignal<Option<Program>>,
    events: EventsTx,
) -> impl IntoView {
    move || {
        let state = match ExecutionContext::from_program(program) {
            Ok(function) => function,
            Err(error) => {
                return view! {
                    <p>{error}</p>
                }
                .into_view();
            }
        };

        // Without this, this closure turns from an `Fn` into an `FnOnce`, which
        // then isn't a `leptos::View`. Not sure why this is needed. Leptos does
        // some magic for the component with children here, and that's what's
        // causing it.
        let events = events.clone();

        view! {
            <Panel>
                <Function
                    program=program
                    function=state.function
                    events=events.clone() />
            </Panel>
        }
    }
}
