use std::path::Path;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

pub fn watch(path: impl AsRef<Path>) -> anyhow::Result<RecommendedWatcher> {
    let mut watcher = notify::recommended_watcher(|event| {
        let _ = dbg!(event);
    })?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    Ok(watcher)
}
