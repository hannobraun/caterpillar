use std::path::PathBuf;

use notify::{RecursiveMode, Watcher as _};
use tokio::sync::watch;
use tracing::error;

use super::debounce::DebouncedChanges;

pub struct Watcher {
    // This field is not used, but we need to keep it around. If we drop the
    // `notify` watcher, it stops watching.
    _watcher: notify::RecommendedWatcher,
    pub changes: DebouncedChanges,
}

impl Watcher {
    pub fn new(crates_dir: PathBuf) -> anyhow::Result<Self> {
        let (tx, rx) = watch::channel(());

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
        watcher.watch(&crates_dir, RecursiveMode::Recursive)?;

        let changes = DebouncedChanges::new(rx);

        Ok(Self {
            _watcher: watcher,
            changes,
        })
    }
}
