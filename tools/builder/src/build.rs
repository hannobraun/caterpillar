use tokio::{
    process::{Child, Command},
    task,
};

use crate::watch::DebouncedChanges;

pub async fn build(changes: DebouncedChanges) {
    task::spawn(watch_and_build(changes));
}

async fn watch_and_build(mut changes: DebouncedChanges) -> anyhow::Result<()> {
    let mut trunk_process: Option<Child> = None;

    build_once(&mut trunk_process).await?;

    while changes.wait_for_change().await {
        build_once(&mut trunk_process).await?;
    }

    Ok(())
}

async fn build_once(trunk_process: &mut Option<Child>) -> anyhow::Result<()> {
    let new_process = Command::new("trunk")
        .arg("serve")
        .args(["--ignore", "."])
        .spawn()?;

    if let Some(mut old_process) = trunk_process.take() {
        old_process.kill().await?;
    }

    *trunk_process = Some(new_process);

    Ok(())
}
