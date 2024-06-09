use std::path::Path;

use notify::{RecursiveMode, Watcher as _};
use tokio::sync::mpsc;
use tracing::error;

use super::{debounce::DebouncedChanges, filter::FilteredChanges};

pub struct Watcher {
    _watcher: notify::RecommendedWatcher,
    pub changes: DebouncedChanges,
}

impl Watcher {
    pub fn new() -> anyhow::Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();

        // We interpret the intent behind calling this function as wanting to
        // "load" the game code, as opposed to just wanting to watch it after
        // possibly having or not having loaded it via other means.
        //
        // Therefore, we need to trigger an initial change.
        tx.send(())?;

        let mut watcher = notify::recommended_watcher(move |event| {
            let _ = match event {
                Ok(event) => event,
                Err(err) => {
                    error!("Error watching for changes: {err}");
                    return;
                }
            };

            if tx.send(()).is_err() {
                // The other end has hung up. Not much we can do about that. The
                // thread this is running on will probably also end soon.
            }
        })?;
        watcher.watch(Path::new("capi"), RecursiveMode::Recursive)?;

        let changes = FilteredChanges::new(rx);
        let changes = DebouncedChanges::new(changes);

        Ok(Self {
            _watcher: watcher,
            changes,
        })
    }
}
