use capi_protocol::command::Command;
use leptos::{component, view, IntoView};

use crate::ui::{components::panel::Panel, send_command, CommandsTx};

#[component]
pub fn ControlPanel(commands: CommandsTx) -> impl IntoView {
    let event_continue = Command::Continue { and_stop_at: None };

    view! {
        <Panel class="">
            <Button
                label="Reset"
                command=Command::Reset
                commands=commands.clone() />
            <Button
                label="Stop"
                command=Command::Stop
                commands=commands.clone() />
            <Button
                label="Continue"
                command=event_continue
                commands=commands.clone() />
            <Button
                label="Step Into"
                command=Command::Step
                commands=commands />
        </Panel>
    }
}

#[component]
fn Button(
    label: &'static str,
    command: Command,
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
