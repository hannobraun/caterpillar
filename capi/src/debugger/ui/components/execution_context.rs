use leptos::{component, view, IntoView, Memo, ReadSignal, SignalGet};

use crate::{
    debugger::{
        model::{ExecutionContext, Function},
        ui::{components::function::Function, EventsTx},
    },
    process::Process,
};

use super::panel::Panel;

#[component]
pub fn ExecutionContext(
    process: ReadSignal<Option<Process>>,
    state: Memo<ExecutionContext>,
    events: EventsTx,
) -> impl IntoView {
    move || {
        // Without this, this closure turns from an `Fn` into an `FnOnce`, which
        // then isn't a `leptos::View`. Not sure why this is needed. Leptos does
        // some magic for the component with children here, and that's what's
        // causing it.
        let events = events.clone();

        let process = process.get()?;
        let state = state.get();

        let function = state.function.map(|function| {
            let class = if state.message.is_some() {
                "blur-sm"
            } else {
                ""
            };

            let function = Function::new(function, &process);

            view! {
                <div class=class>
                    <Function
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

        Some(view! {
            <Panel class="h-80">
                {function}
                {message}
            </Panel>
        })
    }
}
