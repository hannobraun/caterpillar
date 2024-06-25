use std::path::Path;

use notify::{RecursiveMode, Watcher as _};
use tokio::sync::mpsc;
use tracing::error;

use super::debounce::DebouncedChanges;

pub struct Watcher {
    _watcher: notify::RecommendedWatcher,
    changes: DebouncedChanges,
}

impl Watcher {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut watcher = notify::recommended_watcher(move |event| {
            if let Err(err) = event {
                error!("Error watching for changes: {err}");
                return;
            }

            if tx.send(()).is_err() {
                // The other end has hung up. Not much we can do about that. The
                // thread this is running on will probably also end soon.
            }
        })?;
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

        let changes = DebouncedChanges::new(rx);

        Ok(Self {
            _watcher: watcher,
            changes,
        })
    }

    pub fn changes(&self) -> DebouncedChanges {
        self.changes.clone()
    }
}
