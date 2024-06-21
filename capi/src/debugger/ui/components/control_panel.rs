use leptos::{component, view, IntoView};

use crate::debugger::{
    model::DebugCommand,
    ui::{components::panel::Panel, send_event, CommandsTx},
};

#[component]
pub fn ControlPanel(commands: CommandsTx) -> impl IntoView {
    let event_continue = DebugCommand::Continue { and_stop_at: None };

    view! {
        <Panel class="">
            <Button
                label="Reset"
                command=DebugCommand::Reset
                commands=commands.clone() />
            <Button
                label="Stop"
                command=DebugCommand::Stop
                commands=commands.clone() />
            <Button
                label="Continue"
                command=event_continue
                commands=commands.clone() />
            <Button
                label="Step Into"
                command=DebugCommand::Step
                commands=commands />
        </Panel>
    }
}

#[component]
fn Button(
    label: &'static str,
    command: DebugCommand,
    commands: CommandsTx,
) -> impl IntoView {
    let on_click = move |_| {
        leptos::spawn_local(send_event(command.clone(), commands.clone()));
    };

    view! {
        <input
            type="button"
            value=label
            class="m-1 px-1 bg-gray-300 font-bold"
            on:click=on_click />
    }
}
