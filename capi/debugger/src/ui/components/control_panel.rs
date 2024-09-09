use capi_protocol::command::CommandToRuntime;
use leptos::{component, view, IntoView};

use crate::ui::{components::panel::Panel, send_command, CommandsTx};

#[component]
pub fn ControlPanel(commands: CommandsTx) -> impl IntoView {
    view! {
        <Panel class="">
            <Button
                label="Reset"
                command=CommandToRuntime::Reset
                commands=commands.clone() />
            <Button
                label="Stop"
                command=CommandToRuntime::Stop
                commands=commands.clone() />
            <Button
                label="Continue"
                command=CommandToRuntime::Continue
                commands=commands.clone() />
            <Button
                label="Step Into"
                command=CommandToRuntime::Step
                commands=commands />
        </Panel>
    }
}

#[component]
fn Button(
    label: &'static str,
    command: CommandToRuntime,
    commands: CommandsTx,
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
