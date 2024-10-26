use std::path::PathBuf;

use capi_build_game::build_and_watch_game;
use capi_watch::Watcher;

use crate::server;

pub async fn start(address: String, serve_dir: PathBuf) -> anyhow::Result<()> {
    let watcher = Watcher::new(PathBuf::from("games"))?;
    let build_events = build_and_watch_game("snake", watcher.changes).await?;
    server::start(address, serve_dir, build_events).await?;

    Ok(())
}
