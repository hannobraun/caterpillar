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

    pub fn load(
        &mut self,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<(String, Receiver<String>)> {
        let path = path.as_ref();

        let code = load(path)?;
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

fn watch(path: &Path) -> anyhow::Result<ScriptWatcher> {
    let path_for_watcher = PathBuf::from(path);

    let (sender, receiver) = crossbeam_channel::bounded(0);

    let mut debouncer = notify_debouncer_mini::new_debouncer(
        Duration::from_millis(50),
        move |result: DebounceEventResult| {
            let path = &path_for_watcher;

            if let Ok(events) = result {
                for event in events {
                    if let DebouncedEventKind::Any = event.kind {
                        let code = load(path).unwrap();
                        sender.send(code).unwrap();
                    }
                }

                return;
            }

            // Not sure what else we can do about it here.
            error!("Error watching code: {result:?}");
        },
    )?;

    debouncer
        .watcher()
        .watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    Ok(ScriptWatcher {
        updates: receiver,
        watcher: debouncer,
    })
}
