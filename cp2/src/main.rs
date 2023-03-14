mod event_loop;
mod terminal;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    event_loop::run().await?;
    Ok(())
}
