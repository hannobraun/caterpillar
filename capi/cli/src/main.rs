mod headless;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    headless::run().await?;
    Ok(())
}
