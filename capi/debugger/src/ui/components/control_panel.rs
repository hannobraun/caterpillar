use leptos::{component, view, IntoView};

use crate::{
    model::UserAction,
    ui::{actions::send_action, components::panel::Panel, ActionsTx},
};

#[component]
pub fn ControlPanel(actions: ActionsTx) -> impl IntoView {
    view! {
        <Panel class="">
            <Button
                label="Reset"
                action=UserAction::Reset
                actions=actions.clone() />
            <Button
                label="Stop"
                action=UserAction::Stop
                actions=actions.clone() />
            <Button
                label="Continue"
                action=UserAction::Continue
                actions=actions.clone() />
            <Button
                label="Step Into"
                action=UserAction::Step
                actions=actions />
        </Panel>
    }
}

#[component]
fn Button(
    label: &'static str,
    action: UserAction,
    actions: ActionsTx,
) -> impl IntoView {
    let on_click = move |_| {
        leptos::spawn_local(send_action(action.clone(), actions.clone()));
    };

    view! {
        <input
            type="button"
            value=label
            class="m-1 px-1 bg-gray-300 font-bold"
            on:click=on_click />
    }
}
