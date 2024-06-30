use capi_watch::Watcher;

mod args;
mod build;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = args::Args::parse();
    let watcher = Watcher::new(std::path::PathBuf::from("games"))?;
    let game_rx = build::build_and_watch(watcher.changes).await?;
    server::start(args.address, args.serve_dir, game_rx).await?;

    tracing::info!("`capi-server` shutting down.");
    Ok(())
}
