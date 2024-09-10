use leptos::ReadSignal;

use crate::{
    debugger::{PersistentState, TransientState},
    ui::components::debugger::Debugger,
};

use super::ActionsTx;

pub fn init(
    debugger: ReadSignal<(PersistentState, TransientState)>,
    actions: ActionsTx,
) {
    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger
                debugger=debugger
                actions=actions />
        }
    });
}
