use tokio::process::{Child, Command};

use crate::watch::DebouncedChanges;

pub async fn start(changes: DebouncedChanges) {
    watch_and_build(changes).await.unwrap();
}

async fn watch_and_build(mut changes: DebouncedChanges) -> anyhow::Result<()> {
    let mut trunk_process: Option<Child> = None;

    build_once(&mut trunk_process).await?;

    while changes.wait_for_change().await {
        if !build_once(&mut trunk_process).await? {
            break;
        }
    }

    Ok(())
}

async fn build_once(trunk_process: &mut Option<Child>) -> anyhow::Result<bool> {
    let new_process = Command::new("trunk")
        .arg("serve")
        .args(["--ignore", "."])
        .spawn()?;

    if let Some(mut old_process) = trunk_process.take() {
        old_process.kill().await?;
    }

    *trunk_process = Some(new_process);

    Ok(true)
}
