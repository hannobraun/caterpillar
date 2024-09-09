use leptos::ReadSignal;

use crate::{debugger::Debugger, ui::components::debugger::Debugger};

use super::ActionsTx;

pub fn init(debugger: ReadSignal<Debugger>, actions: ActionsTx) {
    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger
                debugger=debugger
                actions=actions />
        }
    });
}
