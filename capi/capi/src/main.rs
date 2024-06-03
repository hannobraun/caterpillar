mod display;
mod effects;
mod runner;
mod server;
mod snake;
mod updates;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http::trace=info")
        .init();

    let mut script = capi_runtime::Script::default();
    snake::snake(&mut script);
    let program = capi_runtime::compile(script, "main");

    let (events_tx, events_rx) = tokio::sync::mpsc::unbounded_channel();
    let (updates_tx, updates_rx) = tokio::sync::watch::channel(program.clone());

    let updates_tx = updates::UpdatesTx::new(updates_tx);

    server::start(updates_rx, events_tx);
    let runner = runner::RunnerThread::start(program, events_rx, updates_tx);
    display::run(runner)
}
