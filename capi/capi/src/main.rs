mod capi;
mod display;
mod server;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http::trace=info")
        .init();

    let (program, functions) = capi::Program::new();

    let (events_tx, events_rx) = tokio::sync::mpsc::unbounded_channel();

    server::start(functions.clone(), events_tx);
    display::run(program, functions, events_rx)
}
