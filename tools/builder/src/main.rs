mod build;
mod pipeline;
mod serve;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    pipeline::pipeline().await?;
    Ok(())
}
