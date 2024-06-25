use std::path::Path;

use notify::{RecursiveMode, Watcher as _};
use tokio::sync::mpsc;
use tracing::error;

use super::debounce::DebouncedChanges;

pub struct Watcher {
    // This field is not used, but we need to keep it around. If we drop the
    // `notify` watcher, it stops watching.
    _watcher: notify::RecommendedWatcher,
    changes: DebouncedChanges,
}

impl Watcher {
    pub fn new(crates_dir: impl AsRef<Path>) -> anyhow::Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut watcher = notify::recommended_watcher(move |event| {
            let event = match event {
                Ok(event) => event,
                Err(err) => {
                    error!("Error watching for changes: {err}");
                    return;
                }
            };

            if tx.send(event).is_err() {
                // The other end has hung up. Not much we can do about that. The
                // thread this is running on will probably also end soon.
            }
        })?;
        watcher.watch(crates_dir.as_ref(), RecursiveMode::Recursive)?;

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
