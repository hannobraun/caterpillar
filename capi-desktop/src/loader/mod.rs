mod script_loader;

use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Context;
use crossbeam_channel::{Receiver, SendError};
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{
    DebounceEventResult, DebouncedEventKind, Debouncer,
};
use tracing::error;

use self::script_loader::ScriptLoader;

#[derive(Default)]
pub struct Loader {
    watchers: Vec<Debouncer<RecommendedWatcher>>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            watchers: Vec::new(),
        }
    }

    /// Load the script at the given path
    ///
    /// # Implementation Note
    ///
    /// It would be better to not return the code as a `String` here, and
    /// instead trigger the first update through the channel. Then we only have
    /// to support one code path for loading code.
    pub fn load(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> anyhow::Result<(String, Receiver<anyhow::Result<String>>)> {
        let path = path.into();

        let code = load(&path)?;
        let ScriptWatcher { updates, watcher } = watch(path)?;

        self.watchers.push(watcher);
        Ok((code, updates))
    }
}

struct ScriptWatcher {
    pub updates: Receiver<anyhow::Result<String>>,
    pub watcher: Debouncer<RecommendedWatcher>,
}

fn load(path: &Path) -> anyhow::Result<String> {
    let code = load_inner(path)
        .with_context(|| format!("Loading script `{}`", path.display()))?;
    Ok(code)
}

fn load_inner(path: &Path) -> io::Result<String> {
    let mut code = String::new();
    File::open(path)?.read_to_string(&mut code)?;
    Ok(code)
}

fn watch(path: PathBuf) -> anyhow::Result<ScriptWatcher> {
    let path_for_watcher = path.clone();

    let (sender, receiver) = crossbeam_channel::bounded(0);
    let script_loader = ScriptLoader::new(path_for_watcher, sender);

    let mut debouncer = notify_debouncer_mini::new_debouncer(
        Duration::from_millis(50),
        move |result: DebounceEventResult| {
            let events = match result {
                Ok(events) => events,
                Err(err) => {
                    if let Err(SendError(err)) = script_loader.on_error(err) {
                        // If we end up here, the channel has been disconnected.
                        // Nothing we can do about it here, but log the error.
                        //
                        // If the channel is disconnected, the watcher should
                        // have been dropped too, which would mean this whole
                        // thread is about to die anyway.
                        error!("Failed to send code watching error: {err:?}");
                    }
                    return;
                }
            };

            for event in events {
                if let DebouncedEventKind::Any = event.kind {
                    if let Err(SendError(result)) = script_loader.on_change() {
                        // See comment above on why this is the appropriate way
                        // to handle this.
                        error!(
                            "Failed to send code loading result: {result:?}"
                        );
                    }
                }
            }
        },
    )?;

    debouncer
        .watcher()
        .watch(&path, RecursiveMode::NonRecursive)?;

    Ok(ScriptWatcher {
        updates: receiver,
        watcher: debouncer,
    })
}
