use capi_protocol::command::CommandToRuntime;
use leptos::{component, view, IntoView};

use crate::ui::{components::panel::Panel, send_command, ActionsTx};

#[component]
pub fn ControlPanel(actions: ActionsTx) -> impl IntoView {
    view! {
        <Panel class="">
            <Button
                label="Reset"
                command=CommandToRuntime::Reset
                actions=actions.clone() />
            <Button
                label="Stop"
                command=CommandToRuntime::Stop
                actions=actions.clone() />
            <Button
                label="Continue"
                command=CommandToRuntime::Continue
                actions=actions.clone() />
            <Button
                label="Step Into"
                command=CommandToRuntime::Step
                actions=actions />
        </Panel>
    }
}

#[component]
fn Button(
    label: &'static str,
    command: CommandToRuntime,
    actions: ActionsTx,
) -> impl IntoView {
    let on_click = move |_| {
        leptos::spawn_local(send_command(command.clone(), actions.clone()));
    };

    view! {
        <input
            type="button"
            value=label
            class="m-1 px-1 bg-gray-300 font-bold"
            on:click=on_click />
    }
}
