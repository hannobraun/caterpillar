use leptos::{component, view, IntoView};

use crate::debugger::{
    model::ExecutionContext,
    ui::{components::function::Function, EventsTx},
};

use super::panel::Panel;

#[component]
pub fn ExecutionContext(
    execution_context: ExecutionContext,
    events: EventsTx,
) -> impl IntoView {
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

    view! {
        <Panel class="h-80">
            {function}
            {message}
        </Panel>
    }
}
