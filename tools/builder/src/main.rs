mod build;
mod serve;
mod watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let watcher = watch::Watcher::new()?;
    serve::start(watcher.changes()).await?;
    build::start(watcher.changes());

    Ok(())
}
