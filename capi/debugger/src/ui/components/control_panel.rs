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
                commands=actions.clone() />
            <Button
                label="Stop"
                command=CommandToRuntime::Stop
                commands=actions.clone() />
            <Button
                label="Continue"
                command=CommandToRuntime::Continue
                commands=actions.clone() />
            <Button
                label="Step Into"
                command=CommandToRuntime::Step
                commands=actions />
        </Panel>
    }
}

#[component]
fn Button(
    label: &'static str,
    command: CommandToRuntime,
    commands: ActionsTx,
) -> impl IntoView {
    let on_click = move |_| {
        leptos::spawn_local(send_command(command.clone(), commands.clone()));
    };

    view! {
        <input
            type="button"
            value=label
            class="m-1 px-1 bg-gray-300 font-bold"
            on:click=on_click />
    }
}
