use capi_protocol::command::CommandToRuntime;
use leptos::{component, view, IntoView};

use crate::ui::{components::panel::Panel, send_command, Action, ActionsTx};

#[component]
pub fn ControlPanel(actions: ActionsTx) -> impl IntoView {
    view! {
        <Panel class="">
            <Button
                label="Reset"
                action=Action::Reset
                actions=actions.clone() />
            <Button
                label="Stop"
                action=Action::Stop
                actions=actions.clone() />
            <Button
                label="Continue"
                action=Action::Continue
                actions=actions.clone() />
            <Button
                label="Step Into"
                action=Action::Step
                actions=actions />
        </Panel>
    }
}

#[component]
fn Button(
    label: &'static str,
    action: Action,
    actions: ActionsTx,
) -> impl IntoView {
    let command = match action {
        Action::BreakpointClear { instruction } => {
            CommandToRuntime::BreakpointClear { instruction }
        }
        Action::BreakpointSet { instruction } => {
            CommandToRuntime::BreakpointSet { instruction }
        }
        Action::Continue => CommandToRuntime::Continue,
        Action::Reset => CommandToRuntime::Reset,
        Action::Step => CommandToRuntime::Step,
        Action::Stop => CommandToRuntime::Stop,
    };

    let on_click = move |_| {
        leptos::spawn_local(send_command(command.clone(), actions.clone()));
    };

    view! {
        <input
            type="button"
            value=label
            class="m-1 px-1 bg-gray-300 font-bold"
            on:click=on_click />
    }
}
