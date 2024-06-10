use tokio::process::{Child, Command};

use crate::watch::DebouncedChanges;

pub async fn build(mut changes: DebouncedChanges) -> anyhow::Result<()> {
    let mut trunk_process: Option<Child> = None;

    while changes.wait_for_change().await {
        single_build(&mut trunk_process).await?;
    }

    Ok(())
}

async fn single_build(trunk_process: &mut Option<Child>) -> anyhow::Result<()> {
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
