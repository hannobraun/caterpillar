mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http::trace=info")
        .init();

    let program = capi_runtime::games::build(capi_runtime::games::snake::snake);

    let (updates_tx, updates_rx) =
        capi_runtime::updates::updates(program.clone());

    let (events_tx, runner) = {
        let (events_tx, handle, start) =
            capi_runtime::runner::runner(program, updates_tx);
        std::thread::spawn(start);
        (events_tx, handle)
    };

    server::start(updates_rx, events_tx);
    capi_runtime::display::run(runner).await?;

    Ok(())
}
