mod client;
mod ui;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging to console");

    leptos::spawn_local(main_async());
}

async fn main_async() {
    let (events_tx, events_rx) = futures::channel::mpsc::unbounded();

    let set_program = ui::start(events_tx);
    leptos::spawn_local(client::handle_server(set_program, events_rx));

    log::info!("Caterpillar initialized.");
}
