mod display;
mod runner;
mod server;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http::trace=info")
        .init();

    let program = capi_runtime::games::build(capi_runtime::games::snake::snake);

    let (updates_tx, updates_rx) =
        capi_runtime::updates::updates(program.clone());

    let (events_tx, runner) = runner::runner(program, updates_tx);
    server::start(updates_rx, events_tx);
    display::run(runner)
}
