use capi_runtime::DebugEvent;
use leptos::{component, view, IntoView};
use web_sys::MouseEvent;

use crate::{
    client::{send_event, EventsTx},
    components::panel::Panel,
};

#[component]
pub fn ControlPanel(events: EventsTx) -> impl IntoView {
    view! {
        <Panel>
            <ResetButton
                events=events />
        </Panel>
    }
}

#[component]
fn ResetButton(events: EventsTx) -> impl IntoView {
    let send_reset = move |_| {
        leptos::spawn_local(send_event(DebugEvent::Reset, events.clone()));
    };

    view! {
        <Button
            on_click=send_reset />
    }
}

#[component]
fn Button<F>(on_click: F) -> impl IntoView
where
    F: FnMut(MouseEvent) + 'static,
{
    view! {
        <input
            type="button"
            value="Reset"
            class="m-1 px-1 bg-gray-300 font-bold"
            on:click=on_click />
    }
}
