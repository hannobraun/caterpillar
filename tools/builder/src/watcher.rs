use std::path::Path;

use notify::{RecursiveMode, Watcher};
use tokio::sync::mpsc;

pub fn watch() -> anyhow::Result<mpsc::UnboundedReceiver<()>> {
    let (tx, rx) = mpsc::unbounded_channel();

    let mut watcher = notify::recommended_watcher(move |_| {
        if tx.send(()).is_err() {
            // The other end has hung up. Not much we can do about that. The
            // thread this is running on will probably also end soon.
        }
    })?;
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    Ok(rx)
}
