mod build;
mod pipelines;
mod serve;
mod watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    pipelines::pipeline().await?;

    Ok(())
}
