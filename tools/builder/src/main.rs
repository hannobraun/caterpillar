mod watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut watcher = watch::watch()?;

    use tokio_stream::StreamExt;
    while let Some(event) = watcher.changes.next().await {
        dbg!(event);
    }

    Ok(())
}
