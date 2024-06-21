use leptos::{component, view, IntoView};

use crate::debugger::{
    model::DebugCommand,
    ui::{components::panel::Panel, send_event, EventsTx},
};

#[component]
pub fn ControlPanel(events: EventsTx) -> impl IntoView {
    let event_continue = DebugCommand::Continue { and_stop_at: None };

    view! {
        <Panel class="">
            <Button
                label="Reset"
                event=DebugCommand::Reset
                events=events.clone() />
            <Button
                label="Stop"
                event=DebugCommand::Stop
                events=events.clone() />
            <Button
                label="Continue"
                event=event_continue
                events=events.clone() />
            <Button
                label="Step Into"
                event=DebugCommand::Step
                events=events />
        </Panel>
    }
}

#[component]
fn Button(
    label: &'static str,
    event: DebugCommand,
    events: EventsTx,
) -> impl IntoView {
    let on_click = move |_| {
        leptos::spawn_local(send_event(event.clone(), events.clone()));
    };

    view! {
        <input
            type="button"
            value=label
            class="m-1 px-1 bg-gray-300 font-bold"
            on:click=on_click />
    }
}
