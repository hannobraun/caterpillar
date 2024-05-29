use capi_runtime::debugger::DebugEvent;
use leptos::{component, view, IntoView};

use crate::{
    client::{send_event, EventsTx},
    components::panel::Panel,
};

#[component]
pub fn ControlPanel(events: EventsTx) -> impl IntoView {
    view! {
        <Panel class="">
            <Button
                label="Stop"
                event=DebugEvent::Stop
                events=events.clone() />
            <Button
                label="Continue"
                event=DebugEvent::Continue
                events=events.clone() />
            <Button
                label="Step Into"
                event=DebugEvent::Step
                events=events.clone() />
            <Button
                label="Reset"
                event=DebugEvent::Reset
                events=events />
        </Panel>
    }
}

#[component]
fn Button(
    label: &'static str,
    event: DebugEvent,
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
