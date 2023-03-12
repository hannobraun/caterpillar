mod terminal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let frame_time = std::time::Duration::from_millis(125);
    terminal::run(frame_time, || eprintln!("Hello, world!")).await?;
    Ok(())
}
