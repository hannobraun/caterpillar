use std::path::PathBuf;

use capi_build_game::{build_and_watch_game, CompilerOutput, Event};
use capi_protocol::Versioned;
use capi_watch::Watcher;
use tokio::sync::watch;

use crate::server;

pub async fn start(address: String, serve_dir: PathBuf) -> anyhow::Result<()> {
    let watcher = Watcher::new(PathBuf::from("games"))?;
    let mut build_events =
        build_and_watch_game("snake", watcher.changes).await?;

    let Some(Event::ChangeDetected) = build_events.recv().await else {
        // The channel has been closed already. This means we're shutting down.
        return Ok(());
    };
    println!("build:change");
    let Some(Event::BuildFinished(code)) = build_events.recv().await else {
        // The channel has been closed already. This means we're shutting down.
        return Ok(());
    };
    println!("build:finish");
    let (code_tx, code_rx) = watch::channel(code);

    server::start(address, serve_dir, code_rx);

    while let Some(event) = build_events.recv().await {
        match event {
            capi_build_game::Event::ChangeDetected => {
                println!("build:change");
            }
            capi_build_game::Event::BuildFinished(code) => {
                println!("build:finish");

                if code_tx.send(code).is_err() {
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

pub type CodeRx = watch::Receiver<Versioned<CompilerOutput>>;
