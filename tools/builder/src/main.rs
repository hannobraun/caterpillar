mod build;
mod serve;
mod watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let watcher = watch::Watcher::new()?;
    serve::serve(watcher.changes()).await?;
    build::build(watcher.changes()).await;

    Ok(())
}
