mod terminal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    terminal::run().await?;
    Ok(())
}
