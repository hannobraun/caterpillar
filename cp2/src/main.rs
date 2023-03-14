mod event_loop;
mod terminal;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let frame_time = std::time::Duration::from_millis(125);
    let mut buffer = ui::Buffer::new();
    let mut stdout = std::io::stdout();

    terminal::run(frame_time, |size| {
        std::future::ready(event_loop::run_once(size, &mut buffer, &mut stdout))
    })
    .await?;

    Ok(())
}
