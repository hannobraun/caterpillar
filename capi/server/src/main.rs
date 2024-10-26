use std::path::PathBuf;

use capi_build_game::build_and_watch_game;
use capi_watch::Watcher;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    let watcher = Watcher::new(PathBuf::from("games"))?;
    let game = build_and_watch_game("snake", watcher.changes).await?;
    capi_server::start(args.address, args.serve_dir, game).await?;

    tracing::info!("`capi-server` shutting down.");
    Ok(())
}

/// Caterpillar server
#[derive(clap::Parser)]
pub struct Args {
    /// Address to serve at
    #[arg(short, long)]
    pub address: String,

    /// Directory to serve from
    #[arg(short, long)]
    pub serve_dir: PathBuf,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
