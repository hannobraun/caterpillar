use leptos::{component, view, IntoView, Memo, SignalGet};

use crate::debugger::{
    model::ExecutionContext,
    ui::{components::function::Function, EventsTx},
};

use super::panel::Panel;

#[component]
pub fn ExecutionContext(
    execution_context: Memo<ExecutionContext>,
    events: EventsTx,
) -> impl IntoView {
    move || {
        // Without this, this closure turns from an `Fn` into an `FnOnce`, which
        // then isn't a `leptos::View`. Not sure why this is needed. Leptos does
        // some magic for the component with children here, and that's what's
        // causing it.
        let events = events.clone();

        let execution_context = execution_context.get();

        let function = execution_context.function.map(|function| {
            let class = if execution_context.message.is_some() {
                "blur-sm"
            } else {
                ""
            };

            view! {
                <div class=class>
                    <Function
                        function=function
                        events=events.clone() />
                </div>
            }
        });
        let message = execution_context.message.map(|message| {
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
