use std::path::PathBuf;

use capi_build_game::build_and_watch_game;
use capi_watch::Watcher;

use crate::server::{self, CodeTx};

pub async fn start(address: String, serve_dir: PathBuf) -> anyhow::Result<()> {
    let watcher = Watcher::new(PathBuf::from("games"))?;
    let mut build_events =
        build_and_watch_game("snake", watcher.changes).await?;

    let mut server_task = ServerTask::Uninitialized { address, serve_dir };

    while let Some(event) = build_events.recv().await {
        match event {
            capi_build_game::Event::ChangeDetected => {
                println!("build:change");
            }
            capi_build_game::Event::BuildFinished(code) => {
                println!("build:finish");

                match server_task {
                    ServerTask::Uninitialized { address, serve_dir } => {
                        let (ready_rx, code_tx) =
                            server::start(address, serve_dir, code);

                        ready_rx.await?;
                        println!("ready"); // signal the builder we're ready

                        server_task = ServerTask::Initialized { code_tx };
                    }
                    ServerTask::Initialized { code_tx } => {
                        if code_tx.send(code).is_err() {
                            return Ok(());
                        }
                        server_task = ServerTask::Initialized { code_tx };
                    }
                }
            }
        }
    }

    Ok(())
}

enum ServerTask {
    Uninitialized { address: String, serve_dir: PathBuf },
    Initialized { code_tx: CodeTx },
}
