use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Context;
use crossbeam_channel::Receiver;
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{
    DebounceEventResult, DebouncedEventKind, Debouncer,
};
use tracing::error;

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
    ) -> anyhow::Result<(String, Receiver<String>)> {
        let path = path.into();

        let code = load(&path)?;
        let ScriptWatcher { updates, watcher } = watch(path)?;

        self.watchers.push(watcher);
        Ok((code, updates))
    }
}

struct ScriptWatcher {
    pub updates: Receiver<String>,
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

    let mut debouncer = notify_debouncer_mini::new_debouncer(
        Duration::from_millis(50),
        move |result: DebounceEventResult| {
            let path = &path_for_watcher;

            match result {
                Ok(events) => {
                    for event in events {
                        if let DebouncedEventKind::Any = event.kind {
                            let code = load(path).unwrap();
                            sender.send(code).unwrap();
                        }
                    }
                }
                Err(err) => {
                    // Not sure what else we can do about it here.
                    error!("Error watching code: {err:?}");
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
