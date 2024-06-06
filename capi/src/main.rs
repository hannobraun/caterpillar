mod display;
mod effects;
mod runner;
mod server;
mod updates;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http::trace=info")
        .init();

    let program = capi_runtime::games::build(capi_runtime::games::snake::snake);

    let (events_tx, events_rx) = tokio::sync::mpsc::unbounded_channel();

    let (updates_tx, updates_rx) = updates::UpdatesTx::new(program.clone());

    server::start(updates_rx, events_tx);
    let runner = runner::RunnerThread::start(program, events_rx, updates_tx);
    display::run(runner)
}
