use leptos::{component, view, IntoView};

use crate::debugger::{
    model::DebugCommand,
    ui::{components::panel::Panel, send_event, CommandsTx},
};

#[component]
pub fn ControlPanel(events: CommandsTx) -> impl IntoView {
    let event_continue = DebugCommand::Continue { and_stop_at: None };

    view! {
        <Panel class="">
            <Button
                label="Reset"
                command=DebugCommand::Reset
                events=events.clone() />
            <Button
                label="Stop"
                command=DebugCommand::Stop
                events=events.clone() />
            <Button
                label="Continue"
                command=event_continue
                events=events.clone() />
            <Button
                label="Step Into"
                command=DebugCommand::Step
                events=events />
        </Panel>
    }
}

#[component]
fn Button(
    label: &'static str,
    command: DebugCommand,
    events: CommandsTx,
) -> impl IntoView {
    let on_click = move |_| {
        leptos::spawn_local(send_event(command.clone(), events.clone()));
    };

    view! {
        <input
            type="button"
            value=label
            class="m-1 px-1 bg-gray-300 font-bold"
            on:click=on_click />
    }
}
