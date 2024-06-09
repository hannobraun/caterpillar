mod watcher;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut watcher = watcher::watch()?;

    while let Some(event) = watcher.channel.recv().await {
        dbg!(event);
    }

    Ok(())
}
