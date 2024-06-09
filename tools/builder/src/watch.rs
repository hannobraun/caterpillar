use std::path::Path;

use notify::{RecursiveMode, Watcher as _};
use tokio::sync::mpsc;

pub fn watch() -> anyhow::Result<Watcher> {
    let (tx, rx) = mpsc::unbounded_channel();

    let mut watcher = notify::recommended_watcher(move |_| {
        if tx.send(()).is_err() {
            // The other end has hung up. Not much we can do about that. The
            // thread this is running on will probably also end soon.
        }
    })?;
    watcher.watch(Path::new("capi"), RecursiveMode::Recursive)?;

    Ok(Watcher {
        _watcher: watcher,
        channel: rx,
    })
}

pub struct Watcher {
    _watcher: notify::RecommendedWatcher,
    pub channel: mpsc::UnboundedReceiver<()>,
}
