mod build;
mod pipeline;
mod serve;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("builder::build=debug,builder::serve=debug")
        .init();
    pipeline::pipeline().await?;
    Ok(())
}
