#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    use notify::Watcher;
    let mut watcher = notify::recommended_watcher(move |event| {
        if tx.send(event).is_err() {
            // The other end has hung up. Not much we can do about that. The
            // thread this is running on will probably also end soon.
        }
    })?;
    watcher
        .watch(std::path::Path::new("."), notify::RecursiveMode::Recursive)?;

    while let Some(event) = rx.recv().await {
        let _ = dbg!(event);
    }

    Ok(())
}
