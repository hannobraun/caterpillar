use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Context;
use notify::{RecursiveMode, Watcher as _};
use tokio::sync::mpsc;
use tracing::error;

use super::debounce::DebouncedChanges;

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
        watcher.watch(Path::new("capi"), RecursiveMode::Recursive)?;

        let changes = DebouncedChanges::new(rx);

        Ok(Self {
            _watcher: watcher,
            changes,
        })
    }
}

fn ignore_event(paths: Vec<PathBuf>) -> anyhow::Result<bool> {
    let current_dir = env::current_dir()?
        .canonicalize()
        .context("Canonicalize current directory")?;

    let mut ignore_all = true;
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
            ignore_all &= path.starts_with("capi/dist");
        }
    }

    Ok(ignore_all)
}
