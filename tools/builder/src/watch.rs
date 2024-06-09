use std::path::Path;

use tokio::sync::mpsc;

pub fn watch(
) -> anyhow::Result<mpsc::UnboundedReceiver<notify::Result<notify::Event>>> {
    let (tx, rx) = mpsc::unbounded_channel();

    use notify::Watcher;
    let mut watcher = notify::recommended_watcher(move |event| {
        if tx.send(event).is_err() {
            // The other end has hung up. Not much we can do about that. The
            // thread this is running on will probably also end soon.
        }
    })?;
    watcher.watch(Path::new("."), notify::RecursiveMode::Recursive)?;

    Ok(rx)
}
