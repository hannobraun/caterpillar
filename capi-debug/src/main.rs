mod client;
mod ui;

use futures::channel::mpsc;
use leptos::{create_signal, view};

use crate::client::handle_server;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging to console");

    let (program, set_program) = create_signal(None);
    let (events_tx, events_rx) = mpsc::unbounded();

    leptos::spawn_local(handle_server(set_program, events_rx));

    leptos::mount_to_body(
        move || view! { <ui::debugger::Debugger program=program events=events_tx /> },
    );

    log::info!("Caterpillar initialized.");
}
