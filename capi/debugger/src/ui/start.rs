use leptos::ReadSignal;

use crate::{debugger::Debugger, ui::components::debugger::Debugger};

use super::CommandsTx;

pub fn start(debugger: ReadSignal<Debugger>, commands: CommandsTx) {
    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger
                debugger=debugger
                commands=commands />
        }
    });
}
