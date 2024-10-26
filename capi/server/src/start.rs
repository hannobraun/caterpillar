use std::path::PathBuf;

use capi_build_game::{build_and_watch_game, CompilerOutput};
use capi_protocol::Versioned;
use capi_watch::Watcher;
use tokio::{sync::watch, task};

use crate::server;

pub async fn start(address: String, serve_dir: PathBuf) -> anyhow::Result<()> {
    let watcher = Watcher::new(PathBuf::from("games"))?;
    let mut build_events =
        build_and_watch_game("snake", watcher.changes).await?;

    let Some(code) = build_events.recv().await else {
        // The channel has been closed already. This means we're shutting down.
        return Ok(());
    };
    let (code_tx, code_rx) = watch::channel(code);

    task::spawn(async move {
        while let Some(event) = build_events.recv().await {
            if code_tx.send(event).is_err() {
                return;
            }
        }
    });

    server::start(address, serve_dir, code_rx).await?;

    Ok(())
}

pub type CodeRx = watch::Receiver<Versioned<CompilerOutput>>;
