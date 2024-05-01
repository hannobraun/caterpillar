mod capi;
mod display;
mod server;
mod tiles;
mod updates;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http::trace=info")
        .init();

    let program = capi::program();

    let (events_tx, events_rx) = tokio::sync::mpsc::unbounded_channel();
    let (updates_tx, updates_rx) = tokio::sync::watch::channel(program.clone());

    let updates_tx = updates::UpdatesTx::new(updates_tx);

    server::start(updates_rx, events_tx);
    display::run(program, events_rx, updates_tx)
}
