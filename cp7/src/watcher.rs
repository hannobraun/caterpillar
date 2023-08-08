use std::path::Path;

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

pub fn watch(path: impl AsRef<Path>) -> anyhow::Result<RecommendedWatcher> {
    let mut watcher =
        notify::recommended_watcher(|result: Result<Event, _>| {
            if let Ok(event) = result {
                if let EventKind::Modify(_) = event.kind {
                    dbg!(event);
                }

                return;
            }

            // Not sure what else we can do about it here.
            eprintln!("Error watching code: {result:?}");
        })?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    Ok(watcher)
}
