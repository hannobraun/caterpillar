mod capi;
mod display;
mod server;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http::trace=info")
        .init();

    let (program, functions) = capi::Program::new();

    let (events_tx, mut events_rx) = tokio::sync::mpsc::unbounded_channel();

    std::thread::spawn(move || {
        while let Some(event) = events_rx.blocking_recv() {
            dbg!(event);
        }
    });

    server::start(functions, events_tx);
    display::run(program)
}
