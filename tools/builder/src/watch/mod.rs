mod debounce;

use std::path::Path;

use notify::{RecursiveMode, Watcher as _};
use tokio::sync::mpsc;

pub fn watch() -> anyhow::Result<Watcher> {
    let (tx, rx) = mpsc::unbounded_channel();

    // We interpret the intent behind calling this function as wanting to "load"
    // the game code, as opposed to just wanting to watch it after possibly
    // having or not having loaded it via other means.
    //
    // Therefore, we need to trigger an initial change.
    tx.send(())?;

    let mut watcher = notify::recommended_watcher(move |_| {
        if tx.send(()).is_err() {
            // The other end has hung up. Not much we can do about that. The
            // thread this is running on will probably also end soon.
        }
    })?;
    watcher.watch(Path::new("capi"), RecursiveMode::Recursive)?;

    let changes = debounce::DebouncedChanges::new(rx);

    Ok(Watcher::new(watcher, changes))
}

pub struct Watcher {
    _watcher: notify::RecommendedWatcher,
    pub changes: debounce::DebouncedChanges,
}

impl Watcher {
    fn new(
        watcher: notify::RecommendedWatcher,
        changes: debounce::DebouncedChanges,
    ) -> Self {
        Self {
            _watcher: watcher,
            changes,
        }
    }
}
