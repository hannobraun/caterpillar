mod event_loop;
mod terminal;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let frame_time = std::time::Duration::from_millis(125);
    let mut buffer = ui::Buffer::new();

    terminal::run(frame_time, |size, stdout| {
        std::future::ready(event_loop::run_once(size, &mut buffer, stdout))
    })
    .await?;

    Ok(())
}
