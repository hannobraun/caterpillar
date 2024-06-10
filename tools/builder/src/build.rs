use tokio::process::Command;

use crate::watch::DebouncedChanges;

pub async fn start(changes: DebouncedChanges) {
    watch_and_build(changes).await.unwrap();
}

async fn watch_and_build(mut changes: DebouncedChanges) -> anyhow::Result<()> {
    build_once().await?;

    while changes.wait_for_change().await {
        if !build_once().await? {
            break;
        }
    }

    Ok(())
}

async fn build_once() -> anyhow::Result<bool> {
    Command::new("trunk").arg("build").status().await?;
    Ok(true)
}
