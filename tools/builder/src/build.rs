use tokio::process::{Child, Command};
use tokio_stream::{Stream, StreamExt};

pub async fn build(
    mut changes: impl Stream<Item = ()> + Unpin,
) -> anyhow::Result<()> {
    let mut trunk_process: Option<Child> = None;

    while let Some(()) = changes.next().await {
        let new_process = Command::new("trunk")
            .arg("serve")
            .args(["--ignore", "."])
            .spawn()?;

        if let Some(mut old_process) = trunk_process.take() {
            old_process.kill().await?;
        }

        trunk_process = Some(new_process);
    }

    Ok(())
}
