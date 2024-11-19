use std::path::PathBuf;

use tempfile::tempdir;

use super::Event;

#[tokio::test]
async fn basic_build() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let games_dir = PathBuf::from("../../games");
    let address = "[::1]:34481".parse()?;
    let serve_dir = tempdir()?;

    let mut events = crate::server::start(
        games_dir,
        address,
        serve_dir.path().to_path_buf(),
    )
    .await?;

    // Wait for server to be ready.
    while let Some(event) = events.recv().await {
        if let Event::ServerReady = event {
            break;
        }
    }

    reqwest::get("http://[::1]:34481/code").await?;

    Ok(())
}
