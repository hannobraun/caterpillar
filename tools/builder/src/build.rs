use std::process;

use tokio::{process::Command, sync::watch, task};
use tracing::error;

use crate::watch::DebouncedChanges;

pub fn start(changes: DebouncedChanges) -> watch::Receiver<()> {
    let (tx, rx) = watch::channel(());
    task::spawn(async {
        if let Err(err) = watch_and_build(changes, tx).await {
            error!("Build error: {err}");
            process::exit(1);
        }
    });
    rx
}

async fn watch_and_build(
    mut changes: DebouncedChanges,
    updates: watch::Sender<()>,
) -> anyhow::Result<()> {
    build_once(&updates).await?;

    while changes.wait_for_change().await {
        if !build_once(&updates).await? {
            break;
        }
    }

    Ok(())
}

async fn build_once(updates: &watch::Sender<()>) -> anyhow::Result<bool> {
    Command::new("trunk").arg("build").status().await?;

    if updates.send(()).is_err() {
        return Ok(false);
    }

    Ok(true)
}
