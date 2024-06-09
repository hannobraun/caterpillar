use tokio::process::Command;
use tokio_stream::{Stream, StreamExt};

pub async fn build(
    mut changes: impl Stream<Item = ()> + Unpin,
) -> anyhow::Result<()> {
    while let Some(()) = changes.next().await {
        Command::new("trunk")
            .arg("serve")
            .args(["--ignore", "."])
            .spawn()?
            .wait()
            .await?;
    }

    Ok(())
}
