use capi_runtime::DebugEvent;
use leptos::{component, view, IntoView};
use web_sys::MouseEvent;

use crate::{
    client::{send_event, EventsTx},
    components::panel::Panel,
};

#[component]
pub fn ControlPanel(events: EventsTx) -> impl IntoView {
    let send_reset = move |_| {
        leptos::spawn_local(send_event(DebugEvent::Reset, events.clone()));
    };

    view! {
        <Panel>
            <Button
                value="Reset"
                on_click=send_reset />
        </Panel>
    }
}

#[component]
fn Button<F>(value: &'static str, on_click: F) -> impl IntoView
where
    F: FnMut(MouseEvent) + 'static,
{
    view! {
        <input
            type="button"
            value=value
            class="m-1 px-1 bg-gray-300 font-bold"
            on:click=on_click />
    }
}
