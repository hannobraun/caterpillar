mod client;
mod ui;

use futures::channel::mpsc;

use crate::client::handle_server;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging to console");

    let (events_tx, events_rx) = mpsc::unbounded();

    let set_program = ui::start(events_tx);
    leptos::spawn_local(handle_server(set_program, events_rx));

    log::info!("Caterpillar initialized.");
}
