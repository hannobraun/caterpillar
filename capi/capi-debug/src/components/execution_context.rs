use capi_runtime::{ExecutionContext, Program};
use leptos::{component, view, IntoView, Memo, ReadSignal, SignalGet};

use crate::{client::EventsTx, components::function::Function};

use super::panel::Panel;

#[component]
pub fn ExecutionContext(
    program: ReadSignal<Option<Program>>,
    state: Memo<ExecutionContext>,
    events: EventsTx,
) -> impl IntoView {
    move || {
        // Without this, this closure turns from an `Fn` into an `FnOnce`, which
        // then isn't a `leptos::View`. Not sure why this is needed. Leptos does
        // some magic for the component with children here, and that's what's
        // causing it.
        let events = events.clone();

        let state = state.get();

        let function = state.function.map(|function| {
            let class = if state.message.is_some() {
                "blur-sm"
            } else {
                ""
            };

            view! {
                <div class=class>
                    <Function
                        program=program
                        function=function
                        events=events.clone() />
                </div>
            }
        });
        let message = state.message.map(|message| {
            view! {
                <p class="w-full h-full absolute inset-y-0 flex justify-center items-center">
                    {message}
                </p>
            }
        });

        view! {
            <Panel class="h-80">
                {function}
                {message}
            </Panel>
        }
    }
}
