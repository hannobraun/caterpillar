mod watcher;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rx = watcher::watch()?;

    while let Some(event) = rx.channel.recv().await {
        dbg!(event);
    }

    Ok(())
}
