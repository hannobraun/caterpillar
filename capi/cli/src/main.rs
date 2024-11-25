mod build_game;
mod cli;
mod files;
mod headless;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::run().await?;
    Ok(())
}
