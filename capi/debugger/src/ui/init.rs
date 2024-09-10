use leptos::ReadSignal;

use crate::{
    debugger::{PersistentState, TransientState},
    ui::components::debugger::Debugger,
};

use super::ActionsTx;

pub fn init(
    state: ReadSignal<(PersistentState, TransientState)>,
    actions: ActionsTx,
) {
    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger
                state=state
                actions=actions />
        }
    });
}
