mod watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut watcher = watch::watch()?;

    while let Some(event) = watcher.channel.recv().await {
        dbg!(event);
    }

    Ok(())
}
