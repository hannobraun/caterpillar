mod build;
mod watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let watcher = watch::watch()?;
    build::build(watcher.changes).await?;

    Ok(())
}
