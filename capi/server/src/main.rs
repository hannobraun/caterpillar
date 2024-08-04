use capi_watch::Watcher;

mod args;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = args::Args::parse();
    let watcher = Watcher::new(std::path::PathBuf::from("games"))?;
    let game =
        capi_watch::build_and_watch_game("snake", watcher.changes).await?;
    server::start(args.address, args.serve_dir, game).await?;

    tracing::info!("`capi-server` shutting down.");
    Ok(())
}
