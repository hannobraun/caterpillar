mod build;
mod serve;
mod watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let watcher = watch::Watcher::new()?;
    let mut updates = build::start(watcher.changes()).await;
    serve::start(watcher.changes()).await?;

    while let Ok(update) = updates.changed().await {
        dbg!(update);
    }

    Ok(())
}
