use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Context;
use notify::{RecursiveMode, Watcher as _};
use tokio::sync::watch;
use tracing::error;

use super::debounce::DebouncedChanges;

pub struct Watcher {
    _watcher: notify::RecommendedWatcher,
    changes: DebouncedChanges,
}

impl Watcher {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        let (tx, rx) = watch::channel(());

        let mut watcher = notify::recommended_watcher(move |event| {
            let event: notify::Event = match event {
                Ok(event) => event,
                Err(err) => {
                    error!("Error watching for changes: {err}");
                    return;
                }
            };

            match ignore_event(event.paths) {
                Ok(ignore) => {
                    if ignore {
                        return;
                    }
                }
                Err(err) => {
                    error!("Error while filtering path: {err}");
                    return;
                }
            }

            if tx.send(()).is_err() {
                // The other end has hung up. Not much we can do about that. The
                // thread this is running on will probably also end soon.
            }
        })?;
        watcher.watch(path, RecursiveMode::Recursive)?;

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

fn ignore_event(paths: Vec<PathBuf>) -> anyhow::Result<bool> {
    let current_dir = env::current_dir()?
        .canonicalize()
        .context("Canonicalize current directory")?;

    let mut ignore = true;
    for path in paths {
        let path = match path.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                // This happens all the time, if the path doesn't point to a
                // valid file or directory. This is normal, as a removed file is
                // an event, for example.
                continue;
            }
        };

        if let Ok(path) = path.strip_prefix(&current_dir) {
            ignore &= path.starts_with("capi/runtime/dist");
        }
    }

    Ok(ignore)
}
