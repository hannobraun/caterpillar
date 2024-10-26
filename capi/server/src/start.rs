use crate::server;

pub async fn start(
    address: String,
    serve_dir: std::path::PathBuf,
) -> anyhow::Result<()> {
    use std::path::PathBuf;

    use capi_build_game::build_and_watch_game;
    use capi_watch::Watcher;

    let watcher = Watcher::new(PathBuf::from("games"))?;
    let game = build_and_watch_game("snake", watcher.changes).await?;
    server::start(address, serve_dir, game).await?;

    Ok(())
}
