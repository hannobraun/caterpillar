use capi_runtime::DebugEvent;
use leptos::{component, view, IntoView};

use crate::{
    client::{send_event, EventsTx},
    components::panel::Panel,
};

#[component]
pub fn ControlPanel(events: EventsTx) -> impl IntoView {
    view! {
        <Panel>
            <Button
                value="Reset"
                event=DebugEvent::Reset
                events=events.clone() />
            <Button
                value="Continue"
                event=DebugEvent::Continue
                events=events.clone() />
            <Button
                value="Step"
                event=DebugEvent::Step
                events=events />
        </Panel>
    }
}

#[component]
fn Button(
    value: &'static str,
    event: DebugEvent,
    events: EventsTx,
) -> impl IntoView {
    let on_click = move |_| {
        leptos::spawn_local(send_event(event.clone(), events.clone()));
    };

    view! {
        <input
            type="button"
            value=value
            class="m-1 px-1 bg-gray-300 font-bold"
            on:click=on_click />
    }
}
