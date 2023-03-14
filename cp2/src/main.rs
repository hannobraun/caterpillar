mod event_loop;
mod terminal;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let frame_time = std::time::Duration::from_millis(125);

    terminal::run(frame_time, |size, buffer, stdout| {
        std::future::ready(event_loop::run_once(size, buffer, stdout))
    })
    .await?;

    Ok(())
}
